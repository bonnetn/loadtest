use std::{
    future::Future,
    pin::Pin,
    rc::Rc,
    time::{Duration, Instant, SystemTime},
};

use async_channel::{Receiver, Sender};
use compio::time::interval_at;
use futures::{FutureExt as _, StreamExt as _, channel::oneshot, stream::FuturesUnordered};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive as _;

use crate::{
    cli::Args,
    error::{AppError, Result},
    stats::{Statistics, WorkerStats},
    work_unit::{ExecutionResult, RequestWorkUnit, WorkUnit},
};

#[derive(Clone)]
pub struct RunResult {
    pub run_timestamp: SystemTime,
    pub worker_stats: Vec<WorkerStats>,
    pub success_latencies: Vec<Duration>,
    pub non_success_latencies: Vec<Duration>,
}

pub struct Order {
    pub response: oneshot::Sender<Result<ExecutionResult>>,
}

pub fn run_executor(id: usize, args: &Args, receiver: Receiver<Order>) -> Result<()> {
    let runtime = compio::runtime::Runtime::builder().build()?;
    let root = runtime.spawn(executor_task(
        id,
        receiver,
        Rc::new(RequestWorkUnit::new(args)?),
    ));
    runtime
        .block_on(root)
        .map_err(|error| AppError::Io(error.into()))?;
    Ok(())
}

pub async fn run_executor_on_current_runtime(
    id: usize,
    args: Args,
    receiver: Receiver<Order>,
) -> Result<()> {
    executor_task(id, receiver, Rc::new(RequestWorkUnit::new(&args)?)).await;
    Ok(())
}

async fn executor_task(id: usize, receiver: Receiver<Order>, work_unit: Rc<RequestWorkUnit>) {
    let mut requests = FuturesUnordered::new();
    loop {
        if requests.is_empty() {
            let Ok(order) = receiver.recv().await else {
                break;
            };
            requests.push(spawn_request(id, order, Rc::clone(&work_unit)));
        } else {
            futures::select! {
                order = receiver.recv().fuse() => {
                    let Ok(order) = order else { break };
                    requests.push(spawn_request(id, order, Rc::clone(&work_unit)));
                }
                _ = requests.next().fuse() => {}
            }
        }
    }
    while requests.next().await.is_some() {}
}

async fn spawn_request(_id: usize, order: Order, work_unit: Rc<RequestWorkUnit>) {
    let task = compio::runtime::spawn(async move {
        let _ = order.response.send(work_unit.execute().await);
    });
    task.await.ok();
}

type Response = Pin<Box<dyn Future<Output = Result<(usize, ExecutionResult)>> + 'static>>;

pub async fn driver_task(args: Args, senders: Vec<Sender<Order>>) -> Result<RunResult> {
    let workers = senders.len();
    let period = rps_to_period(args.requests_per_second);
    let total = args
        .duration
        .as_nanos()
        .checked_div(period.as_nanos())
        .expect("period is zero") as usize;
    let mut interval = interval_at(Instant::now(), period);
    let mut diagnostics = compio::time::interval(Duration::from_millis(250));
    let start = Instant::now();
    let mut sent = 0usize;
    let mut in_flight = vec![0usize; workers];
    let mut stats: Vec<_> = (0..workers)
        .map(|_| Statistics::new(total / workers + 1))
        .collect();
    let mut responses: FuturesUnordered<Response> = FuturesUnordered::new();
    let mut worker_stats = Vec::new();

    while sent < total || in_flight.iter().any(|count| *count != 0) {
        if sent < total && start.elapsed() < args.duration {
            if responses.is_empty() {
                futures::select! {
                    _ = interval.tick().fuse() => issue_order(&senders, &mut responses, &mut in_flight, &mut sent).await?,
                    _ = diagnostics.tick().fuse() => record_worker_stats(&mut worker_stats, &stats, &in_flight, start, sent),
                }
            } else {
                futures::select! {
                    _ = interval.tick().fuse() => issue_order(&senders, &mut responses, &mut in_flight, &mut sent).await?,
                    response = responses.next().fuse() => collect_response(response, &mut in_flight, &mut stats).await?,
                    _ = diagnostics.tick().fuse() => record_worker_stats(&mut worker_stats, &stats, &in_flight, start, sent),
                }
            }
        } else {
            futures::select! {
                response = responses.next().fuse() => collect_response(response, &mut in_flight, &mut stats).await?,
                _ = diagnostics.tick().fuse() => record_worker_stats(&mut worker_stats, &stats, &in_flight, start, sent),
            }
        }
    }
    drop(senders);
    record_worker_stats(&mut worker_stats, &stats, &in_flight, start, sent);
    let mut success_latencies = Vec::new();
    let mut non_success_latencies = Vec::new();
    for stat in &mut stats {
        success_latencies.append(&mut stat.success_latencies);
        non_success_latencies.append(&mut stat.non_success_latencies);
    }
    Ok(RunResult {
        run_timestamp: SystemTime::now(),
        worker_stats,
        success_latencies,
        non_success_latencies,
    })
}

async fn issue_order(
    senders: &[Sender<Order>],
    responses: &mut FuturesUnordered<Response>,
    in_flight: &mut [usize],
    sent: &mut usize,
) -> Result<()> {
    let worker_id = *sent % senders.len();
    let (sender, receiver) = oneshot::channel();
    senders[worker_id]
        .send(Order { response: sender })
        .await
        .map_err(|error| AppError::Io(std::io::Error::other(error.to_string())))?;
    responses.push(Box::pin(async move {
        let result = receiver
            .await
            .map_err(|_| AppError::Io(std::io::Error::other("request task cancelled")))??;
        Ok((worker_id, result))
    }));
    in_flight[worker_id] += 1;
    *sent += 1;
    Ok(())
}

async fn collect_response(
    response: Option<Result<(usize, ExecutionResult)>>,
    in_flight: &mut [usize],
    stats: &mut [Statistics],
) -> Result<()> {
    let (worker_id, result) = response
        .ok_or_else(|| AppError::Io(std::io::Error::other("response channel closed")))??;
    in_flight[worker_id] -= 1;
    stats[worker_id].add(&result);
    Ok(())
}

fn record_worker_stats(
    out: &mut Vec<WorkerStats>,
    counters: &[Statistics],
    in_flight: &[usize],
    start: Instant,
    sent: usize,
) {
    for (id, counter) in counters.iter().enumerate() {
        out.push(WorkerStats {
            timestamp: SystemTime::now(),
            id,
            elapsed: start.elapsed(),
            request_sent: sent,
            in_flight: in_flight[id],
            informational_response: counter.informational_response,
            successful_response: counter.successful_response,
            redirection_message: counter.redirection_message,
            client_error_response: counter.client_error_response,
            server_error_response: counter.server_error_response,
            other_error_response: counter.other_error_response,
            timeouts: counter.timeouts,
        });
    }
}

pub(crate) fn rps_to_period(rps: Decimal) -> Duration {
    let period = Decimal::ONE
        .checked_div(rps)
        .expect("Period must be greater than 0");
    Duration::from_nanos(
        ((period * Decimal::from(1_000_000_000u64)).ceil())
            .to_u64()
            .expect("period must fit in u64 nanoseconds"),
    )
}
