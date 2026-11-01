use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

use async_channel::{Receiver, Sender};
use futures::{
    FutureExt as _,
    channel::oneshot,
    future::{AbortHandle, Abortable},
    pin_mut, select,
};

const IN_FLIGHT_LOG_INTERVAL: Duration = Duration::from_secs(1);
const ARRIVAL_TIMER_PERIOD: Duration = Duration::from_millis(1);

pub struct Worker {
    receiver: Receiver<WorkerRequest>,
}

#[derive(Default)]
struct WorkerMetrics {
    timer_ticks: Cell<u64>,
    timer_ticks_dropped: Cell<u64>,
    slots_traversed: Cell<u64>,
    max_slots_per_tick: Cell<usize>,
    requests_scheduled: Cell<u64>,
    max_requests_per_tick: Cell<u64>,
    max_catch_up_duration_us: Cell<u64>,
    timer_lateness_samples: Cell<u64>,
    max_timer_lateness_us: Cell<u64>,
    tasks_spawned: Cell<u64>,
    tasks_completed: Cell<u64>,
    in_flight: Cell<usize>,
    max_in_flight: Cell<usize>,
}

struct InFlightGuard {
    metrics: Rc<WorkerMetrics>,
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        self.metrics.in_flight.set(self.metrics.in_flight.get() - 1);
        self.metrics
            .tasks_completed
            .set(self.metrics.tasks_completed.get() + 1);
    }
}

struct ActiveTraffic {
    abort_handle: AbortHandle,
    task: compio::runtime::JoinHandle<TrafficOutcome>,
    response: oneshot::Sender<miette::Result<()>>,
    cancel_responses: Vec<oneshot::Sender<miette::Result<()>>>,
    cancelled: Rc<Cell<bool>>,
}

struct TrafficOutcome {
    result: miette::Result<()>,
    cancelled: bool,
}

enum RequestAction {
    None,
    Start(ActiveTraffic),
    Cancel(oneshot::Sender<miette::Result<()>>),
}

enum ActiveEvent {
    Request(miette::Result<RequestAction>),
    Traffic(Result<TrafficOutcome, compio::runtime::JoinError>),
    Shutdown,
    Tick,
}

impl Worker {
    pub fn new() -> (Self, WorkerHandle) {
        let (sender, receiver) = async_channel::unbounded();
        (Self { receiver }, WorkerHandle { sender })
    }

    pub async fn run(self) -> miette::Result<()> {
        let metrics = Rc::new(WorkerMetrics::default());
        let mut interval = compio::time::interval(IN_FLIGHT_LOG_INTERVAL);
        let mut active_traffic = None;

        loop {
            let receive = self.receiver.recv();
            pin_mut!(receive);

            if active_traffic.is_none() {
                select! {
                    request = receive.fuse() => {
                        let Ok(request) = request else {
                            shutdown_active_traffic(&mut active_traffic).await;
                            log_metrics(&metrics);
                            return Ok(());
                        };
                        match handle_request_and_log_error(
                            request,
                            active_traffic.as_ref(),
                            &metrics,
                        ) {
                            Ok(action) => apply_request_action(action, &mut active_traffic),
                            Err(error) => log_request_error(&error),
                        }
                    }
                    _ = interval.tick().fuse() => {
                        log_metrics(&metrics);
                    }
                }
            } else {
                let mut active_future = (&mut active_traffic
                    .as_mut()
                    .expect("active traffic must exist")
                    .task)
                    .fuse();
                let event = select! {
                    request = receive.fuse() => {
                        match request {
                            Ok(request) => ActiveEvent::Request(handle_request_and_log_error(
                                request,
                                active_traffic.as_ref(),
                                &metrics,
                            )),
                            Err(_) => ActiveEvent::Shutdown,
                        }
                    }
                    result = active_future => ActiveEvent::Traffic(result),
                    _ = interval.tick().fuse() => {
                        log_metrics(&metrics);
                        ActiveEvent::Tick
                    },
                };
                match event {
                    ActiveEvent::Request(result) => match result {
                        Ok(action) => apply_request_action(action, &mut active_traffic),
                        Err(error) => log_request_error(&error),
                    },
                    ActiveEvent::Traffic(task_result) => {
                        finish_active_traffic(&mut active_traffic, task_result);
                    }
                    ActiveEvent::Shutdown => {
                        shutdown_active_traffic(&mut active_traffic).await;
                        log_metrics(&metrics);
                        return Ok(());
                    }
                    ActiveEvent::Tick => {}
                }
            }
        }
    }
}

fn apply_request_action(action: RequestAction, active_traffic: &mut Option<ActiveTraffic>) {
    match action {
        RequestAction::None => {}
        RequestAction::Start(new_active_traffic) => {
            debug_assert!(active_traffic.is_none());
            *active_traffic = Some(new_active_traffic);
        }
        RequestAction::Cancel(response) => {
            active_traffic
                .as_mut()
                .expect("cancel action requires active traffic")
                .cancel_responses
                .push(response);
        }
    }
}

fn finish_active_traffic(
    active_traffic: &mut Option<ActiveTraffic>,
    task_result: Result<TrafficOutcome, compio::runtime::JoinError>,
) {
    let active_traffic = active_traffic
        .take()
        .expect("active traffic must exist when its task finishes");
    let outcome = match task_result {
        Ok(outcome) => outcome,
        Err(error) => TrafficOutcome {
            result: Err(miette::miette!("traffic task failed: {error}")),
            cancelled: false,
        },
    };
    let cancellation_succeeded = outcome.cancelled;
    if let Err(error) = send_response(active_traffic.response, outcome.result) {
        log_request_error(&error);
    }
    for response in active_traffic.cancel_responses {
        let result = if cancellation_succeeded {
            Ok(())
        } else {
            Err(miette::miette!(
                "traffic completed before cancellation was applied"
            ))
        };
        if let Err(error) = send_response(response, result) {
            log_request_error(&error);
        }
    }
}

async fn shutdown_active_traffic(active_traffic: &mut Option<ActiveTraffic>) {
    let Some(active_traffic) = active_traffic.take() else {
        return;
    };
    active_traffic.cancelled.set(true);
    active_traffic.abort_handle.abort();
    let _ = active_traffic.task.await;
    let result = Err(miette::miette!("worker shut down during traffic"));
    if let Err(error) = send_response(active_traffic.response, result) {
        log_request_error(&error);
    }
    for response in active_traffic.cancel_responses {
        if let Err(error) = send_response(response, Err(miette::miette!("worker shut down"))) {
            log_request_error(&error);
        }
    }
}

fn handle_request_and_log_error(
    request: WorkerRequest,
    active_traffic: Option<&ActiveTraffic>,
    metrics: &Rc<WorkerMetrics>,
) -> miette::Result<RequestAction> {
    let result = handle_request(request, active_traffic, metrics);
    if result.is_err() {
        log_metrics(metrics);
    }
    result
}

fn log_request_error(error: &miette::Report) {
    eprintln!("worker request failed: {error}");
}

fn handle_request(
    request: WorkerRequest,
    active_traffic: Option<&ActiveTraffic>,
    metrics: &Rc<WorkerMetrics>,
) -> miette::Result<RequestAction> {
    let WorkerRequest { command, response } = request;

    match command {
        WorkerCommand::SendTraffic {
            traffic_slots,
            start_instant,
        } => {
            if active_traffic.is_some() {
                send_response(
                    response,
                    Err(miette::miette!("worker is already running traffic")),
                )?;
                return Ok(RequestAction::None);
            }

            let (abort_handle, abort_registration) = AbortHandle::new_pair();
            let cancelled = Rc::new(Cell::new(false));
            let task_metrics = Rc::clone(metrics);
            let task_cancelled = Rc::clone(&cancelled);
            let task = compio::runtime::spawn(async move {
                match Abortable::new(
                    send_traffic(
                        traffic_slots,
                        start_instant,
                        Rc::clone(&task_metrics),
                        task_cancelled,
                    ),
                    abort_registration,
                )
                .await
                {
                    Ok(result) => TrafficOutcome {
                        result,
                        cancelled: false,
                    },
                    Err(_) => {
                        wait_for_in_flight(task_metrics).await;
                        TrafficOutcome {
                            result: Err(miette::miette!("traffic cancelled")),
                            cancelled: true,
                        }
                    }
                }
            });
            Ok(RequestAction::Start(ActiveTraffic {
                abort_handle,
                task,
                response,
                cancel_responses: Vec::new(),
                cancelled,
            }))
        }
        WorkerCommand::Cancel => {
            if let Some(active_traffic) = active_traffic {
                active_traffic.abort_handle.abort();
                active_traffic.cancelled.set(true);
                return Ok(RequestAction::Cancel(response));
            }
            // Cancellation is idempotent: there is nothing to cancel when idle.
            send_response(response, Ok(()))?;
            Ok(RequestAction::None)
        }
    }
}

fn send_response(
    response: oneshot::Sender<miette::Result<()>>,
    result: miette::Result<()>,
) -> miette::Result<()> {
    response
        .send(result)
        .map_err(|_| miette::miette!("worker response receiver was dropped"))
}

fn log_metrics(metrics: &Rc<WorkerMetrics>) {
    println!(
        "worker {} metrics: ticks={} dropped_ticks={} slots={} max_slots_per_tick={} scheduled={} max_requests_per_tick={} max_catch_up_us={} lateness_samples={} max_lateness_us={} spawned={} completed={} in_flight={} max_in_flight={}",
        std::thread::current().name().unwrap_or("unnamed"),
        metrics.timer_ticks.get(),
        metrics.timer_ticks_dropped.get(),
        metrics.slots_traversed.get(),
        metrics.max_slots_per_tick.get(),
        metrics.requests_scheduled.get(),
        metrics.max_requests_per_tick.get(),
        metrics.max_catch_up_duration_us.get(),
        metrics.timer_lateness_samples.get(),
        metrics.max_timer_lateness_us.get(),
        metrics.tasks_spawned.get(),
        metrics.tasks_completed.get(),
        metrics.in_flight.get(),
        metrics.max_in_flight.get(),
    );
}

async fn send_traffic(
    traffic_slots: Vec<u16>,
    start_instant: Instant,
    metrics: Rc<WorkerMetrics>,
    cancelled: Rc<Cell<bool>>,
) -> miette::Result<()> {
    let mut arrival_timer = compio::time::interval_at(start_instant, ARRIVAL_TIMER_PERIOD);
    let mut next_slot = 0;
    let mut previous_tick_slot = None;

    loop {
        let now = arrival_timer.tick().await;
        let current_slot = (now.duration_since(start_instant).as_nanos()
            / ARRIVAL_TIMER_PERIOD.as_nanos()) as usize;
        metrics.timer_ticks.set(metrics.timer_ticks.get() + 1);
        if metrics.timer_ticks.get().is_multiple_of(100) {
            let lateness_us = Instant::now().saturating_duration_since(now).as_micros() as u64;
            metrics
                .timer_lateness_samples
                .set(metrics.timer_lateness_samples.get() + 1);
            metrics
                .max_timer_lateness_us
                .set(metrics.max_timer_lateness_us.get().max(lateness_us));
        }
        if let Some(previous_tick_slot) = previous_tick_slot {
            let dropped_ticks = current_slot.saturating_sub(previous_tick_slot + 1);
            metrics
                .timer_ticks_dropped
                .set(metrics.timer_ticks_dropped.get() + dropped_ticks as u64);
        }
        previous_tick_slot = Some(current_slot);

        let slots_to_traverse = current_slot.saturating_sub(next_slot) + 1;
        metrics
            .slots_traversed
            .set(metrics.slots_traversed.get() + slots_to_traverse as u64);
        metrics
            .max_slots_per_tick
            .set(metrics.max_slots_per_tick.get().max(slots_to_traverse));
        let catch_up_started_at = (slots_to_traverse > 1).then(Instant::now);
        let mut requests_this_tick = 0;
        while next_slot <= current_slot && next_slot < traffic_slots.len() {
            let request_count = traffic_slots[next_slot] as u64;
            requests_this_tick += request_count;
            metrics
                .requests_scheduled
                .set(metrics.requests_scheduled.get() + request_count);
            for _ in 0..traffic_slots[next_slot] {
                if cancelled.get() {
                    return Err(miette::miette!("traffic cancelled"));
                }
                let in_flight = metrics.in_flight.get() + 1;
                metrics.in_flight.set(in_flight);
                metrics
                    .max_in_flight
                    .set(metrics.max_in_flight.get().max(in_flight));
                metrics.tasks_spawned.set(metrics.tasks_spawned.get() + 1);
                let task_metrics = Rc::clone(&metrics);
                let task_cancelled = Rc::clone(&cancelled);
                compio::runtime::spawn(async move {
                    let _in_flight = InFlightGuard {
                        metrics: task_metrics,
                    };
                    if !task_cancelled.get() {
                        std::hint::black_box(());
                    }
                })
                .detach();
            }
            next_slot += 1;
        }
        metrics
            .max_requests_per_tick
            .set(metrics.max_requests_per_tick.get().max(requests_this_tick));
        if let Some(started_at) = catch_up_started_at {
            let duration_us = Instant::now()
                .saturating_duration_since(started_at)
                .as_micros() as u64;
            metrics
                .max_catch_up_duration_us
                .set(metrics.max_catch_up_duration_us.get().max(duration_us));
        }
        if next_slot >= traffic_slots.len() {
            break;
        }
    }

    wait_for_in_flight(metrics).await;
    Ok(())
}

async fn wait_for_in_flight(metrics: Rc<WorkerMetrics>) {
    while metrics.in_flight.get() != 0 {
        compio::time::sleep(Duration::from_millis(1)).await;
    }
}

#[derive(Debug)]
pub enum WorkerCommand {
    SendTraffic {
        traffic_slots: Vec<u16>,
        start_instant: Instant,
    },
    Cancel,
}

#[derive(Debug)]
struct WorkerRequest {
    command: WorkerCommand,
    response: oneshot::Sender<miette::Result<()>>,
}

#[derive(Debug, Clone)]
pub struct WorkerHandle {
    sender: Sender<WorkerRequest>,
}

impl WorkerHandle {
    pub async fn send(&self, command: WorkerCommand) -> miette::Result<()> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send(WorkerRequest {
                command,
                response: response_sender,
            })
            .await
            .map_err(|_| miette::miette!("worker is no longer available"))?;
        response_receiver
            .await
            .map_err(|_| miette::miette!("worker dropped its response"))?
    }
}
