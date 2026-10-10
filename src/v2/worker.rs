use std::{
    cell::Cell,
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use async_channel::{Receiver, Sender};
use futures::{FutureExt as _, channel::oneshot, pin_mut, select};
use rand::{SeedableRng, rngs::SmallRng};
use rand_distr::{Distribution, Exp};

use super::cancellation::CancellationToken;

const SIMULATED_WORK_DURATION: Duration = Duration::from_secs(1);
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
        let cancellation = CancellationToken::new();
        let random = Rc::new(RefCell::new(SmallRng::from_os_rng()));
        let in_flight = Rc::new(Cell::new(0));
        let logger_in_flight = Rc::clone(&in_flight);
        let logger_cancellation = cancellation.clone();
        compio::runtime::spawn(async move {
            let mut interval = compio::time::interval(IN_FLIGHT_LOG_INTERVAL);
            loop {
                let cancelled = logger_cancellation.cancelled();
                pin_mut!(cancelled);
                select! {
                    _ = cancelled.fuse() => break,
                    _ = interval.tick().fuse() => println!(
                        "worker {} in-flight tasks: {}",
                        std::thread::current().name().unwrap_or("unnamed"),
                        logger_in_flight.get()
                    ),
                }
            }
        })
        .detach();

        loop {
            let receive = self.receiver.recv();
            pin_mut!(receive);
            let request = select! {
                request = receive.fuse() => request,
                _ = cancellation.cancelled().fuse() => break,
            };
            let Ok(request) = request else { break };
            let WorkerRequest { command, response } = request;
            match command {
                WorkerCommand::SendTraffic {
                    arrival_distribution,
                    start_instant,
                    end_instant,
                } => {
                    let task_cancellation = cancellation.clone();
                    let task_in_flight = Rc::clone(&in_flight);
                    let task_random = Rc::clone(&random);
                    compio::runtime::spawn(async move {
                        let result = send_traffic(
                            arrival_distribution,
                            start_instant,
                            end_instant,
                            task_random,
                            task_in_flight,
                            task_cancellation,
                        )
                        .await;
                        let _ = response.send(result);
                    })
                    .detach();
                    continue;
                }
                WorkerCommand::Cancel => {
                    cancellation.cancel();
                    let _ = response.send(Ok(()));
                }
            }
        }
        Ok(())
    }
}

async fn send_traffic(
    arrival_distribution: Exp<f64>,
    start_instant: Instant,
    end_instant: Instant,
    random: Rc<RefCell<SmallRng>>,
    in_flight: Rc<Cell<usize>>,
    cancellation: CancellationToken,
) -> miette::Result<()> {
    if end_instant < start_instant {
        return Err(miette::miette!(
            "end instant must not precede start instant"
        ));
    }

    if Instant::now() < start_instant && !sleep_until_or_cancel(start_instant, &cancellation).await
    {
        return Ok(());
    }

    loop {
        if cancellation.is_cancelled() {
            break;
        }
        let now = Instant::now();
        if now >= end_instant {
            break;
        }

        let delay = Duration::from_secs_f64(arrival_distribution.sample(&mut *random.borrow_mut()));
        let next_arrival = now.checked_add(delay).unwrap_or(end_instant);
        if next_arrival >= end_instant {
            sleep_until_or_cancel(end_instant, &cancellation).await;
            break;
        }

        if !sleep_until_or_cancel(next_arrival, &cancellation).await {
            break;
        }
        in_flight.set(in_flight.get() + 1);
        let task_in_flight = Rc::clone(&in_flight);
        let task_cancellation = cancellation.clone();
        compio::runtime::spawn(async move {
            sleep_until_or_cancel(Instant::now() + SIMULATED_WORK_DURATION, &task_cancellation)
                .await;
            task_in_flight.set(task_in_flight.get() - 1);
        })
        .detach();
    }

    Ok(())
}

async fn sleep_until_or_cancel(deadline: Instant, cancellation: &CancellationToken) -> bool {
    let sleep = compio::time::sleep_until(deadline);
    let cancelled = cancellation.cancelled();
    pin_mut!(sleep, cancelled);
    select! {
        _ = sleep.fuse() => true,
        _ = cancelled.fuse() => false,
    }
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
