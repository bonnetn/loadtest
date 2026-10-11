use std::time::{Duration, Instant};

use futures::future::{join_all, try_join_all};
use rand::{SeedableRng, rngs::SmallRng};
use rand_distr::{Distribution as _, Poisson};

use super::worker::WorkerCommand;
use super::worker::WorkerHandle;

const TRAFFIC_SLOT: Duration = Duration::from_millis(1);

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
        total_throughput: f64,
        start_instant: Instant,
        end_instant: Instant,
    ) -> miette::Result<()> {
        if end_instant < start_instant {
            return Err(miette::miette!(
                "end instant must not precede start instant"
            ));
        }
        if self.handles.is_empty() {
            return Err(miette::miette!("controller has no workers"));
        }

        let slot_count = end_instant
            .duration_since(start_instant)
            .as_nanos()
            .div_ceil(TRAFFIC_SLOT.as_nanos()) as usize;
        let per_worker_rate = total_throughput / self.handles.len() as f64;
        let distribution = Poisson::new(per_worker_rate * TRAFFIC_SLOT.as_secs_f64())
            .map_err(|error| miette::miette!("failed to create traffic distribution: {error}"))?;
        let mut random = rand::make_rng::<SmallRng>();
        let mut schedules = Vec::with_capacity(self.handles.len());
        for _ in &self.handles {
            let mut worker_random = random.fork();
            let mut traffic_slots = Vec::with_capacity(slot_count);
            for _ in 0..slot_count {
                let count = distribution.sample(&mut worker_random);
                traffic_slots.push(
                    u16::try_from(count as u64)
                        .map_err(|_| miette::miette!("traffic count exceeds u16 capacity"))?,
                );
            }
            schedules.push(traffic_slots);
        }

        try_join_all(
            self.handles
                .iter()
                .zip(schedules)
                .map(|(handle, traffic_slots)| {
                    handle.send(WorkerCommand::SendTraffic {
                        traffic_slots,
                        start_instant,
                    })
                }),
        )
        .await
        .map(|_| ())
    }

    pub async fn cancel_workers(&self) -> miette::Result<()> {
        let results = join_all(
            self.handles
                .iter()
                .map(|handle| handle.send(WorkerCommand::Cancel)),
        )
        .await;
        results.into_iter().try_for_each(|result| result)
    }
}
