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

struct ActiveTraffic {
    abort_handle: AbortHandle,
    task: compio::runtime::JoinHandle<()>,
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
                        if let RequestOutcome::Stop = handle_request_and_log_stop(
                            request,
                            &mut active_traffic,
                            &metrics,
                        )? {
                            break;
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
                let traffic_finished = select! {
                    request = receive.fuse() => {
                        if let RequestOutcome::Stop = handle_request_and_log_stop(
                            request,
                            &mut active_traffic,
                            &metrics,
                        )? {
                            break;
                        }
                        false
                    }
                    _ = active_future => true,
                    _ = interval.tick().fuse() => {
                        log_metrics(&metrics);
                        false
                    },
                };
                if traffic_finished {
                    active_traffic = None;
                }
            }
        }
        Ok(())
    }
}

enum RequestOutcome {
    Continue,
    Stop,
}

fn handle_request_and_log_stop(
    request: Result<WorkerRequest, async_channel::RecvError>,
    active_traffic: &mut Option<ActiveTraffic>,
    metrics: &Rc<WorkerMetrics>,
) -> miette::Result<RequestOutcome> {
    let outcome = handle_request(request, active_traffic, metrics)?;
    if matches!(outcome, RequestOutcome::Stop) {
        log_metrics(metrics);
    }
    Ok(outcome)
}

fn handle_request(
    request: Result<WorkerRequest, async_channel::RecvError>,
    active_traffic: &mut Option<ActiveTraffic>,
    metrics: &Rc<WorkerMetrics>,
) -> miette::Result<RequestOutcome> {
    let Ok(WorkerRequest { command, response }) = request else {
        return Ok(RequestOutcome::Stop);
    };

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
            } else {
                let (abort_handle, abort_registration) = AbortHandle::new_pair();
                let task_metrics = Rc::clone(metrics);
                let task = compio::runtime::spawn(async move {
                    let result = Abortable::new(
                        send_traffic(traffic_slots, start_instant, task_metrics),
                        abort_registration,
                    )
                    .await
                    .unwrap_or(Ok(()));
                    let _ = response.send(result);
                });
                *active_traffic = Some(ActiveTraffic { abort_handle, task });
            }
        }
        WorkerCommand::Cancel => {
            if let Some(active_traffic) = active_traffic {
                active_traffic.abort_handle.abort();
            }
            send_response(response, Ok(()))?;
        }
    }
    Ok(RequestOutcome::Continue)
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
) -> miette::Result<()> {
    let mut arrival_timer = compio::time::interval_at(start_instant, ARRIVAL_TIMER_PERIOD);
    let mut next_slot = 0;
    let mut previous_tick_slot = None;

    loop {
        let now = arrival_timer.tick().await;
        let current_slot = (now.duration_since(start_instant).as_nanos()
            / ARRIVAL_TIMER_PERIOD.as_nanos()) as usize;
        metrics.timer_ticks.set(metrics.timer_ticks.get() + 1);
        if metrics.timer_ticks.get() % 100 == 0 {
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
                let in_flight = metrics.in_flight.get() + 1;
                metrics.in_flight.set(in_flight);
                metrics
                    .max_in_flight
                    .set(metrics.max_in_flight.get().max(in_flight));
                metrics.tasks_spawned.set(metrics.tasks_spawned.get() + 1);
                let task_metrics = Rc::clone(&metrics);
                compio::runtime::spawn(async move {
                    std::hint::black_box(());
                    task_metrics.in_flight.set(task_metrics.in_flight.get() - 1);
                    task_metrics
                        .tasks_completed
                        .set(task_metrics.tasks_completed.get() + 1);
                })
                .detach();
            }
            next_slot += 1;
        }
        if next_slot >= traffic_slots.len() {
            break;
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
    }

    Ok(())
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
