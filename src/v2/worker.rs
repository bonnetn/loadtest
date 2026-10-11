use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    rc::Rc,
    time::{Duration, Instant},
};

use async_channel::{Receiver, Sender};
use futures::{
    FutureExt as _,
    channel::oneshot,
    future::{AbortHandle, Abortable},
    pin_mut, select,
    stream::{FuturesUnordered, StreamExt as _},
};

const IN_FLIGHT_LOG_INTERVAL: Duration = Duration::from_secs(1);
const ARRIVAL_TIMER_PERIOD: Duration = Duration::from_millis(1);

pub struct Worker {
    receiver: Receiver<WorkerRequest>,
}

impl Worker {
    pub fn new() -> (Self, WorkerHandle) {
        let (sender, receiver) = async_channel::unbounded();
        (Self { receiver }, WorkerHandle { sender })
    }

    pub async fn run(self) -> miette::Result<()> {
        let in_flight = Rc::new(Cell::new(0));
        let mut interval = compio::time::interval(IN_FLIGHT_LOG_INTERVAL);
        let mut tasks: FuturesUnordered<WorkerTask> = FuturesUnordered::new();
        let mut abort_handles = Vec::new();

        loop {
            let receive = self.receiver.recv();
            pin_mut!(receive);

            if tasks.is_empty() {
                select! {
                    request = receive.fuse() => {
                        if !handle_request(
                            request,
                            &mut tasks,
                            &mut abort_handles,
                            &in_flight,
                        ) {
                            break;
                        }
                    }
                    _ = interval.tick().fuse() => log_in_flight(&in_flight),
                }
            } else {
                select! {
                    request = receive.fuse() => {
                        if !handle_request(
                            request,
                            &mut tasks,
                            &mut abort_handles,
                            &in_flight,
                        ) {
                            break;
                        }
                    }
                    _ = tasks.next().fuse() => {}
                    _ = interval.tick().fuse() => log_in_flight(&in_flight),
                }
            }
        }
        Ok(())
    }
}

type WorkerTask = Pin<Box<dyn Future<Output = ()> + 'static>>;

fn handle_request(
    request: Result<WorkerRequest, async_channel::RecvError>,
    tasks: &mut FuturesUnordered<WorkerTask>,
    abort_handles: &mut Vec<AbortHandle>,
    in_flight: &Rc<Cell<usize>>,
) -> bool {
    let Ok(WorkerRequest { command, response }) = request else {
        return false;
    };

    match command {
        WorkerCommand::SendTraffic {
            traffic_slots,
            start_instant,
            end_instant,
        } => {
            let (abort_handle, abort_registration) = AbortHandle::new_pair();
            abort_handles.push(abort_handle);
            let task_in_flight = Rc::clone(in_flight);
            tasks.push(Box::pin(async move {
                let result = Abortable::new(
                    send_traffic(traffic_slots, start_instant, end_instant, task_in_flight),
                    abort_registration,
                )
                .await
                .unwrap_or(Ok(()));
                let _ = response.send(result);
            }));
        }
        WorkerCommand::Cancel => {
            for abort_handle in abort_handles.drain(..) {
                abort_handle.abort();
            }
            let _ = response.send(Ok(()));
        }
    }
    true
}

fn log_in_flight(in_flight: &Rc<Cell<usize>>) {
    println!(
        "worker {} in-flight tasks: {}",
        std::thread::current().name().unwrap_or("unnamed"),
        in_flight.get()
    );
}

async fn send_traffic(
    traffic_slots: Vec<u16>,
    start_instant: Instant,
    end_instant: Instant,
    in_flight: Rc<Cell<usize>>,
) -> miette::Result<()> {
    if end_instant < start_instant {
        return Err(miette::miette!(
            "end instant must not precede start instant"
        ));
    }

    let mut arrival_timer = compio::time::interval_at(start_instant, ARRIVAL_TIMER_PERIOD);
    let mut next_slot = 0;

    loop {
        arrival_timer.tick().await;
        let now = Instant::now();
        if now >= end_instant {
            break;
        }

        let current_slot = (now.duration_since(start_instant).as_nanos()
            / ARRIVAL_TIMER_PERIOD.as_nanos()) as usize;
        while next_slot <= current_slot && next_slot < traffic_slots.len() {
            for _ in 0..traffic_slots[next_slot] {
                in_flight.set(in_flight.get() + 1);
                let task_in_flight = Rc::clone(&in_flight);
                compio::runtime::spawn(async move {
                    std::hint::black_box(());
                    task_in_flight.set(task_in_flight.get() - 1);
                })
                .detach();
            }
            next_slot += 1;
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum WorkerCommand {
    SendTraffic {
        traffic_slots: Vec<u16>,
        start_instant: Instant,
        end_instant: Instant,
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
