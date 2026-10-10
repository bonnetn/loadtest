use async_channel::{Receiver, Sender};

pub struct Worker {
    receiver: Receiver<WorkerCommand>,
}

impl Worker {
    pub fn new() -> (Self, WorkerHandle) {
        let (sender, receiver) = async_channel::unbounded();
        (Self { receiver }, WorkerHandle { sender })
    }

    pub async fn run(self) -> miette::Result<()> {
        while self.receiver.recv().await.is_ok() {}
        Ok(())
    }
}

#[derive(Debug)]
pub struct WorkerCommand;

#[derive(Debug, Clone)]
pub struct WorkerHandle {
    sender: Sender<WorkerCommand>,
}

impl WorkerHandle {
    pub async fn send(
        &self,
        command: WorkerCommand,
    ) -> Result<(), async_channel::SendError<WorkerCommand>> {
        self.sender.send(command).await
    }
}
