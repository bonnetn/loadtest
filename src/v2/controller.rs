use std::time::Instant;

use futures::future::try_join_all;
use rand_distr::Exp;

use super::worker::WorkerCommand;
use super::worker::WorkerHandle;

#[derive(Debug, Default)]
pub struct Controller {
    handles: Vec<WorkerHandle>,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_handles(handles: Vec<WorkerHandle>) -> Self {
        Self { handles }
    }

    pub fn handles(&self) -> &[WorkerHandle] {
        &self.handles
    }

    pub fn add_handle(&mut self, handle: WorkerHandle) {
        self.handles.push(handle);
    }

    pub async fn send_traffic(
        &self,
        arrival_distribution: Exp<f64>,
        start_instant: Instant,
        end_instant: Instant,
    ) -> miette::Result<()> {
        try_join_all(self.handles.iter().map(|handle| {
            handle.send(WorkerCommand::SendTraffic {
                arrival_distribution: arrival_distribution.clone(),
                start_instant,
                end_instant,
            })
        }))
        .await
        .map(|_| ())
    }
}
