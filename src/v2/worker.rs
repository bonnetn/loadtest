use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

use async_channel::{Receiver, Sender};
use futures::channel::oneshot;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use rand_distr::{Distribution, Exp};

const SIMULATED_WORK_DURATION: Duration = Duration::from_millis(1);
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
        let mut random = SmallRng::from_os_rng();
        let in_flight = Rc::new(Cell::new(0));
        let logger_in_flight = Rc::clone(&in_flight);
        compio::runtime::spawn(async move {
            let mut interval = compio::time::interval(IN_FLIGHT_LOG_INTERVAL);
            loop {
                interval.tick().await;
                println!(
                    "worker {} in-flight tasks: {}",
                    std::thread::current().name().unwrap_or("unnamed"),
                    logger_in_flight.get()
                );
            }
        })
        .detach();

        while let Ok(request) = self.receiver.recv().await {
            let WorkerRequest { command, response } = request;
            let result = match command {
                WorkerCommand::SendTraffic {
                    arrival_distribution,
                    start_instant,
                    end_instant,
                } => {
                    send_traffic(
                        arrival_distribution,
                        start_instant,
                        end_instant,
                        &mut random,
                        Rc::clone(&in_flight),
                    )
                    .await
                }
            };
            let _ = response.send(result);
        }
        Ok(())
    }
}

async fn send_traffic(
    arrival_distribution: Exp<f64>,
    start_instant: Instant,
    end_instant: Instant,
    random: &mut impl Rng,
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

        let delay = Duration::from_secs_f64(arrival_distribution.sample(random));
        let next_arrival = now.checked_add(delay).unwrap_or(end_instant);
        if next_arrival >= end_instant {
            compio::time::sleep_until(end_instant).await;
            break;
        }

        compio::time::sleep_until(next_arrival).await;
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
