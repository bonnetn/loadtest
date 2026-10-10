use std::time::{Duration, Instant};

use async_channel::{Receiver, Sender};
use futures::channel::oneshot;
use rand::rng;
use rand_distr::{Distribution, Exp};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive as _;

pub struct Worker {
    receiver: Receiver<WorkerRequest>,
}

impl Worker {
    pub fn new() -> (Self, WorkerHandle) {
        let (sender, receiver) = async_channel::unbounded();
        (Self { receiver }, WorkerHandle { sender })
    }

    pub async fn run(self) -> miette::Result<()> {
        while let Ok(request) = self.receiver.recv().await {
            let WorkerRequest { command, response } = request;
            match command {
                WorkerCommand::SendTraffic {
                    throughput,
                    start_instant,
                    end_instant,
                } => send_traffic(throughput, start_instant, end_instant).await?,
            }
            let _ = response.send(());
        }
        Ok(())
    }
}

async fn send_traffic(
    throughput: Decimal,
    start_instant: Instant,
    end_instant: Instant,
) -> miette::Result<()> {
    if throughput <= Decimal::ZERO {
        return Err(miette::miette!("throughput must be greater than zero"));
    }
    if end_instant < start_instant {
        return Err(miette::miette!(
            "end instant must not precede start instant"
        ));
    }

    let rate = throughput
        .to_f64()
        .ok_or_else(|| miette::miette!("throughput cannot be represented as f64"))?;
    let distribution = Exp::new(rate)
        .map_err(|error| miette::miette!("failed to create Poisson distribution: {error}"))?;
    let mut random = rng();
    let mut tasks = Vec::new();

    if Instant::now() < start_instant {
        compio::time::sleep_until(start_instant).await;
    }

    loop {
        let now = Instant::now();
        if now >= end_instant {
            break;
        }

        let delay = Duration::from_secs_f64(distribution.sample(&mut random));
        let next_arrival = now.checked_add(delay).unwrap_or(end_instant);
        if next_arrival >= end_instant {
            compio::time::sleep_until(end_instant).await;
            break;
        }

        compio::time::sleep_until(next_arrival).await;
        tasks.push(compio::runtime::spawn(async {}));
    }

    for task in tasks {
        let _ = task.await;
    }

    Ok(())
}

#[derive(Debug)]
pub enum WorkerCommand {
    SendTraffic {
        throughput: Decimal,
        start_instant: Instant,
        end_instant: Instant,
    },
}

#[derive(Debug)]
struct WorkerRequest {
    command: WorkerCommand,
    response: oneshot::Sender<()>,
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
            .map_err(|_| miette::miette!("worker dropped its response"))?;
        Ok(())
    }
}
