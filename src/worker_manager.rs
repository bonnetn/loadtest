use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use futures::StreamExt as _;
use futures::stream::FuturesUnordered;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive as _;
use tokio::{
    select,
    sync::Barrier,
    task::JoinSet,
    time::{Interval, MissedTickBehavior, interval_at},
};
use tokio_util::sync::CancellationToken;

use crate::{
    cli::Args,
    error::Result,
    stats::{Statistics, WorkerStats},
    work_unit::{ExecutionResult, RequestWorkUnit, WorkUnit},
};

/// Result of a load test run: run timestamp, worker snapshots, and latencies split by success.
#[derive(Clone)]
pub struct RunResult {
    pub run_timestamp: SystemTime,
    pub worker_stats: Vec<WorkerStats>,
    pub success_latencies: Vec<Duration>,
    pub non_success_latencies: Vec<Duration>,
}

pub async fn spawn_workers(args: &Args) -> Result<RunResult> {
    let run_timestamp = SystemTime::now();
    let concurrency = tokio::runtime::Handle::current().metrics().num_workers();

    let start_barrier = Arc::new(Barrier::new(
        concurrency.checked_add(1).expect("concurrency overflow for barrier"),
    ));
    let cancelation_token = CancellationToken::new();

    let mut join_set = JoinSet::new();
    let now = tokio::time::Instant::now();

    let requests_per_second = args.requests_per_second;
    let period = rps_to_period(requests_per_second);

    let concurrency_u128: u128 = concurrency
        .try_into()
        .expect("Concurrency must be less than u128::MAX");

    let iterations = args
        .duration
        .as_nanos()
        .checked_div(period.as_nanos())
        .expect("period is zero");
    let iterations_per_worker = iterations
        .checked_div(concurrency_u128)
        .expect("concurrency is zero");

    let iterations_per_worker: usize = iterations_per_worker
        .try_into()
        .expect("Iterations per worker must be less than usize::MAX");

    for id in 0..concurrency {
        let cancelation_token = cancelation_token.clone();
        let start_barrier = Arc::clone(&start_barrier);
        let work_unit = RequestWorkUnit::new(args)?;
        let mut stats = Statistics::new(iterations_per_worker);

        let mut interval = create_interval(id, now, period, concurrency);
        let load_test_duration = args.duration;
        join_set.spawn(async move {
            Worker {
                id,
                max_iterations: iterations_per_worker,
                load_test_duration,
                start_barrier,
                cancelation_token,
                work_unit,
            }
            .work(&mut interval, &mut stats)
            .await
        });
    }

    start_barrier.wait().await;

    let mut worker_stats = Vec::new();
    let mut success_latencies: Vec<Duration> = Vec::new();
    let mut non_success_latencies: Vec<Duration> = Vec::new();
    while let Some(result) = join_set.join_next().await {
        let result = result??;
        let Some((stats, success, non_success)) = result else {
            unimplemented!();
        };
        worker_stats.extend(stats);
        success_latencies.extend(success);
        non_success_latencies.extend(non_success);
    }

    worker_stats.sort_by_key(|w| (w.timestamp, w.id));

    Ok(RunResult {
        run_timestamp,
        worker_stats,
        success_latencies,
        non_success_latencies,
    })
}

struct Worker<W> {
    id: usize,
    max_iterations: usize,
    load_test_duration: Duration,
    start_barrier: Arc<Barrier>,
    cancelation_token: CancellationToken,
    work_unit: W,
}

impl<W> Worker<W>
where
    W: WorkUnit,
{
    async fn work(
        &self,
        interval: &mut Interval,
        stats: &mut Statistics,
    ) -> Result<Option<(Vec<WorkerStats>, Vec<Duration>, Vec<Duration>)>> {
        self.start_barrier.wait().await;

        // Measure the time the worker started so that we can ignore ticks before that.
        let start_time = tokio::time::Instant::now();

        select! {
            () = self.cancelation_token.cancelled() => {
                Ok(None)
            }
            result = self.do_work(start_time, interval, stats) => {
                Ok(Some(result?))
            }
        }
    }

    async fn do_work(
        &self,
        start: tokio::time::Instant,
        interval: &mut Interval,
        stats: &mut Statistics,
    ) -> Result<(Vec<WorkerStats>, Vec<Duration>, Vec<Duration>)> {
        let mut diagnostics_interval = tokio::time::interval(Duration::from_millis(250));
        let mut fu = FuturesUnordered::new();
        let mut current_iteration: usize = 0;

        let mut worker_stats = Vec::new();

        while current_iteration < self.max_iterations && start.elapsed() < self.load_test_duration {
            let elapsed_ns = start.elapsed().as_nanos();
            let period_ns = interval.period().as_nanos();
            let expected_count: usize = elapsed_ns
                .checked_div(period_ns)
                .and_then(|n| usize::try_from(n).ok())
                .expect("expected iterations fit in usize");

            select! {
                tick = interval.tick() => {
                    if tick < start {
                        continue;
                    }

                    while current_iteration < expected_count && current_iteration < self.max_iterations {
                        fu.push(self.execute_once());
                        current_iteration = current_iteration
                            .checked_add(1)
                            .expect("current_iteration overflow");
                    }
                }
                _ = diagnostics_interval.tick() => {
                    self.capture_diagnostics(stats, &fu, start, &mut worker_stats, current_iteration);
                }
                result = fu.next(), if !fu.is_empty() => {
                    let Some(result) = result else {
                        // No more results, we're done.
                        continue;
                    };
                    let r = result?;
                    stats.add(&r);
                }
            }
        }
        self.capture_diagnostics(stats, &fu, start, &mut worker_stats, current_iteration);

        while let Some(result) = fu.next().await {
            result?;
        }

        let success_latencies = std::mem::take(&mut stats.success_latencies);
        let non_success_latencies = std::mem::take(&mut stats.non_success_latencies);
        Ok((worker_stats, success_latencies, non_success_latencies))
    }

    async fn execute_once(&self) -> Result<ExecutionResult> {
        self.work_unit.execute().await
    }

    fn capture_diagnostics<T>(
        &self,
        stats: &Statistics,
        fu: &FuturesUnordered<T>,
        start: tokio::time::Instant,
        worker_stats: &mut Vec<WorkerStats>,
        current_iteration: usize,
    ) {
        let elapsed = start.elapsed();
        let in_flight = fu.len();
        let Statistics {
            ref successful_response,
            ref informational_response,
            ref redirection_message,
            ref client_error_response,
            ref server_error_response,
            ref other_error_response,
            ref timeouts,
            ..
        } = *stats;
        let id = self.id;

        worker_stats.push(WorkerStats {
            timestamp: SystemTime::now(),
            id,
            elapsed,
            request_sent: current_iteration,
            in_flight,
            successful_response: *successful_response,
            informational_response: *informational_response,
            redirection_message: *redirection_message,
            client_error_response: *client_error_response,
            server_error_response: *server_error_response,
            other_error_response: *other_error_response,
            timeouts: *timeouts,
        });
    }
}

fn create_interval(
    id: usize,
    now: tokio::time::Instant,
    period: Duration,
    concurrency: usize,
) -> Interval {
    let concurrency: u32 = concurrency
        .try_into()
        .expect("Concurrency must be less than u32::MAX");
    let id: u32 = id.try_into().expect("ID must be less than u32::MAX");

    let worker_period = period
        .checked_mul(concurrency)
        .expect("worker period overflow");
    let start = now
        .checked_add(period.checked_mul(id).expect("start offset overflow"))
        .expect("start overflow");

    let mut interval = interval_at(start, worker_period);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    interval
}

fn rps_to_period(rps: Decimal) -> Duration {
    let period = Decimal::ONE
        .checked_div(rps)
        .expect("Period must be greater than 0");
    let billion = Decimal::from(1_000_000_000_u64);
    let period_ns = period
        .checked_mul(billion)
        .expect("Period must be less than 1 billion");
    let period_ns = period_ns
        .to_u64()
        .expect("Period must be less than u64::MAX");
    Duration::from_nanos(period_ns)
}
