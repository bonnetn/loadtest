use super::worker::Worker;

#[derive(Debug, Default)]
pub struct Controller {
    workers: Vec<Worker>,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_workers(workers: Vec<Worker>) -> Self {
        Self { workers }
    }

    pub fn workers(&self) -> &[Worker] {
        &self.workers
    }

    pub fn add_worker(&mut self, worker: Worker) {
        self.workers.push(worker);
    }
}
