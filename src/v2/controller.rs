use super::worker::WorkerChannel;

#[derive(Debug, Default)]
pub struct Controller {
    channels: Vec<WorkerChannel>,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_channels(channels: Vec<WorkerChannel>) -> Self {
        Self { channels }
    }

    pub fn channels(&self) -> &[WorkerChannel] {
        &self.channels
    }

    pub fn add_channel(&mut self, channel: WorkerChannel) {
        self.channels.push(channel);
    }
}
