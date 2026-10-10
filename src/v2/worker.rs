use std::{
    cell::Cell,
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
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
use rand::{SeedableRng, rngs::SmallRng};
use rand_distr::{Distribution, Exp};

const SIMULATED_WORK_DURATION: Duration = Duration::from_secs(1);
const IN_FLIGHT_LOG_INTERVAL: Duration = Duration::from_secs(1);

pub struct Worker {
    receiver: Receiver<WorkerRequest>,
    scheduled: Arc<AtomicU64>,
}

impl Worker {
    pub fn new() -> (Self, WorkerHandle) {
        let (sender, receiver) = async_channel::unbounded();
        (
            Self {
                receiver,
                scheduled: Arc::new(AtomicU64::new(0)),
            },
            WorkerHandle { sender },
        )
    }

    pub async fn run(self) -> miette::Result<()> {
        let random = Rc::new(RefCell::new(SmallRng::from_os_rng()));
        let in_flight = Rc::new(Cell::new(0));
        let scheduled = Arc::clone(&self.scheduled);
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
                            &random,
                            &in_flight,
                            &scheduled,
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
                            &random,
                            &in_flight,
                            &scheduled,
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
    random: &Rc<RefCell<SmallRng>>,
    in_flight: &Rc<Cell<usize>>,
    scheduled: &Arc<AtomicU64>,
) -> bool {
    let Ok(WorkerRequest { command, response }) = request else {
        return false;
    };

    match command {
        WorkerCommand::SendTraffic {
            arrival_distribution,
            start_instant,
            end_instant,
        } => {
            let (abort_handle, abort_registration) = AbortHandle::new_pair();
            abort_handles.push(abort_handle);
            let task_random = Rc::clone(random);
            let task_in_flight = Rc::clone(in_flight);
            let task_scheduled = Arc::clone(scheduled);
            tasks.push(Box::pin(async move {
                let result = Abortable::new(
                    send_traffic(
                        arrival_distribution,
                        start_instant,
                        end_instant,
                        task_random,
                        task_in_flight,
                        task_scheduled,
                    ),
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
    arrival_distribution: Exp<f64>,
    start_instant: Instant,
    end_instant: Instant,
    random: Rc<RefCell<SmallRng>>,
    in_flight: Rc<Cell<usize>>,
    scheduled: Arc<AtomicU64>,
) -> miette::Result<()> {
    if end_instant < start_instant {
        return Err(miette::miette!(
            "end instant must not precede start instant"
        ));
    }

    if Instant::now() < start_instant {
        compio::time::sleep_until(start_instant).await;
    }

    loop {
        let now = Instant::now();
        if now >= end_instant {
            break;
        }

        let delay = Duration::from_secs_f64(arrival_distribution.sample(&mut *random.borrow_mut()));
        let next_arrival = now.checked_add(delay).unwrap_or(end_instant);
        if next_arrival >= end_instant {
            compio::time::sleep_until(end_instant).await;
            break;
        }

        compio::time::sleep_until(next_arrival).await;
        scheduled.fetch_add(1, Ordering::Relaxed);
        in_flight.set(in_flight.get() + 1);
        let task_in_flight = Rc::clone(&in_flight);
        compio::runtime::spawn(async move {
            compio::time::sleep(SIMULATED_WORK_DURATION).await;
            task_in_flight.set(task_in_flight.get() - 1);
        })
        .detach();
    }

    Ok(())
}

#[derive(Debug)]
pub enum WorkerCommand {
    SendTraffic {
        arrival_distribution: Exp<f64>,
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
