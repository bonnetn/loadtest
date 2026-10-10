use std::{
    cell::Cell,
    time::{Duration, Instant},
};

use async_channel::{Receiver, Sender};
use futures::channel::oneshot;
use rand::{Rng, rng};
use rand_distr::{Distribution, Exp};

const SIMULATED_WORK_DURATION: Duration = Duration::from_millis(1);

pub struct Worker {
    receiver: Receiver<WorkerRequest>,
}

impl Worker {
    pub fn new() -> (Self, WorkerHandle) {
        let (sender, receiver) = async_channel::unbounded();
        (Self { receiver }, WorkerHandle { sender })
    }

    pub async fn run(self) -> miette::Result<()> {
        let mut random = rng();
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
        compio::runtime::spawn(async {
            let work = Cell::new(0);
            compio::time::sleep(SIMULATED_WORK_DURATION).await;
            work.set(work.get() + 1);
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
