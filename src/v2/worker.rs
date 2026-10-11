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
use rand::{SeedableRng, rngs::SmallRng};
use rand_distr::{Distribution, Exp};

const IN_FLIGHT_LOG_INTERVAL: Duration = Duration::from_secs(1);

pub struct Worker {
    receiver: Receiver<WorkerRequest>,
}

impl Worker {
    pub fn new() -> (Self, WorkerHandle) {
        let (sender, receiver) = async_channel::unbounded();
        (Self { receiver }, WorkerHandle { sender })
    }

    pub async fn run(self) -> miette::Result<()> {
        let mut random = rand::make_rng::<SmallRng>();
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
                            &mut random,
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
                            &mut random,
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
    random: &mut SmallRng,
    in_flight: &Rc<Cell<usize>>,
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
            let task_random = random.fork();
            let task_in_flight = Rc::clone(in_flight);
            tasks.push(Box::pin(async move {
                let result = Abortable::new(
                    send_traffic(
                        arrival_distribution,
                        start_instant,
                        end_instant,
                        task_random,
                        task_in_flight,
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
    mut random: SmallRng,
    in_flight: Rc<Cell<usize>>,
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

        let delay = Duration::from_secs_f64(arrival_distribution.sample(&mut random));
        let next_arrival = now.checked_add(delay).unwrap_or(end_instant);
        if next_arrival >= end_instant {
            compio::time::sleep_until(end_instant).await;
            break;
        }

        compio::time::sleep_until(next_arrival).await;
        in_flight.set(in_flight.get() + 1);
        let task_in_flight = Rc::clone(&in_flight);
        compio::runtime::spawn(async move {
            std::hint::black_box(());
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
