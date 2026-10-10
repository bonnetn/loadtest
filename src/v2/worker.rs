use std::time::Instant;

use async_channel::{Receiver, Sender};
use futures::channel::oneshot;
use rust_decimal::Decimal;

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
                } => {
                    let _ = (throughput, start_instant, end_instant);
                }
            }
            let _ = response.send(());
        }
        Ok(())
    }
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
