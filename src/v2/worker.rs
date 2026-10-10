use async_channel::{Receiver, Sender};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Worker {
    id: usize,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(self) -> usize {
        self.id
    }

    pub async fn run(self) -> miette::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct WorkerCommand;

#[derive(Debug)]
pub struct WorkerChannel {
    id: usize,
    sender: Sender<WorkerCommand>,
}

impl WorkerChannel {
    pub fn new(id: usize) -> (Self, Receiver<WorkerCommand>) {
        let (sender, receiver) = async_channel::unbounded();
        (Self { id, sender }, receiver)
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub async fn send(
        &self,
        command: WorkerCommand,
    ) -> Result<(), async_channel::SendError<WorkerCommand>> {
        self.sender.send(command).await
    }
}
