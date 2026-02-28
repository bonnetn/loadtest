//! Run report generation (CDF, protobuf encoding). Caller writes the returned bytes (e.g. with `tokio::fs::write`).

use std::time::{Duration, UNIX_EPOCH};

use prost::Message as _;
use rust_decimal::prelude::ToPrimitive as _;

use crate::cdf;
use crate::cli::Args;
use crate::error::Result;
use crate::proto::{
    CdfPoint, Header, LoadTestConfig, LoadTestRunReport, WorkerStats as ProtoWorkerStats,
};
use crate::stats::WorkerStats;
use crate::worker_manager::RunResult;

const START_PERCENTILE: f64 = 1.0;
const RESOLUTION: u64 = 16;
const STEPS: u64 = 4;

/// Builds the run report and returns the serialized protobuf. Prints CDFs to stdout.
/// Caller is responsible for writing the bytes (e.g. `tokio::fs::write(path, &bytes).await`).
pub fn build_run_report(args: &Args, result: &RunResult) -> Result<Vec<u8>> {
    let cdf_success = cdf::calculate_cdf(
        START_PERCENTILE,
        RESOLUTION,
        STEPS,
        result.success_latencies.iter(),
    );

    let cdf_non_success = cdf::calculate_cdf(
        START_PERCENTILE,
        RESOLUTION,
        STEPS,
        result.non_success_latencies.iter(),
    );

    let cdf_all = cdf::calculate_cdf(
        START_PERCENTILE,
        RESOLUTION,
        STEPS,
        result
            .success_latencies
            .iter()
            .chain(result.non_success_latencies.iter()),
    );

    let report = build_proto(args, result, cdf_success, cdf_non_success, cdf_all);

    let buf = report.encode_to_vec();
    Ok(buf)
}

fn build_proto(
    args: &Args,
    result: &RunResult,
    cdf_success: Vec<(f64, Duration)>,
    cdf_non_success: Vec<(f64, Duration)>,
    cdf_all: Vec<(f64, Duration)>,
) -> LoadTestRunReport {
    let run_timestamp_unix_nanos = system_time_to_unix_nanos(result.run_timestamp);

    let config = Some(args_to_proto(args));

    let worker_stats = result
        .worker_stats
        .iter()
        .map(map_worker_stats_to_proto)
        .collect();

    let cdf = cdf_all.into_iter().map(map_cdf_point_to_proto).collect();
    let cdf_success = cdf_success
        .into_iter()
        .map(map_cdf_point_to_proto)
        .collect();
    let cdf_non_success = cdf_non_success
        .into_iter()
        .map(map_cdf_point_to_proto)
        .collect();

    LoadTestRunReport {
        run_timestamp_unix_nanos,
        config,
        worker_stats,
        cdf,
        cdf_success,
        cdf_non_success,
    }
}

/// Converts a `SystemTime` to Unix timestamp in nanoseconds. Panics if before epoch or overflow.
fn system_time_to_unix_nanos(t: std::time::SystemTime) -> i64 {
    t.duration_since(UNIX_EPOCH)
        .expect("run timestamp is before UNIX epoch")
        .as_nanos()
        .try_into()
        .expect("run timestamp overflow (nanos do not fit i64)")
}

fn args_to_proto(args: &Args) -> LoadTestConfig {
    let url = args.url.to_string();
    let method = args.request.as_str().to_owned();
    let requests_per_second = args.requests_per_second.to_u32().unwrap_or(0);
    let duration_secs =
        i64::try_from(args.duration.as_secs()).expect("duration seconds fit in i64");
    let headers = args.header.iter().map(map_header_to_proto).collect();

    LoadTestConfig {
        url,
        method,
        requests_per_second,
        duration_secs,
        headers,
    }
}

fn map_header_to_proto((name, value): (&http::HeaderName, &http::HeaderValue)) -> Header {
    Header {
        name: name.as_str().to_owned(),
        value: value.to_str().unwrap().to_owned(),
    }
}

fn map_worker_stats_to_proto(w: &WorkerStats) -> ProtoWorkerStats {
    let timestamp_unix_nanos = system_time_to_unix_nanos(w.timestamp);
    let worker_id = u32::try_from(w.id).expect("worker id fits in u32");
    let elapsed_nanos = w.elapsed.as_nanos().try_into().unwrap_or(u64::MAX);
    let request_sent = u64::try_from(w.request_sent).expect("request_sent fits in u64");
    let in_flight = u64::try_from(w.in_flight).expect("in_flight fits in u64");
    let informational_response = w.informational_response;
    let successful_response = w.successful_response;
    let redirection_message = w.redirection_message;
    let client_error_response = w.client_error_response;
    let server_error_response = w.server_error_response;
    let other_error_response = w.other_error_response;
    let timeouts = w.timeouts;

    ProtoWorkerStats {
        timestamp_unix_nanos,
        worker_id,
        elapsed_nanos,
        request_sent,
        in_flight,
        informational_response,
        successful_response,
        redirection_message,
        client_error_response,
        server_error_response,
        other_error_response,
        timeouts,
    }
}

fn map_cdf_point_to_proto((p, d): (f64, Duration)) -> CdfPoint {
    CdfPoint {
        percentile: p,
        latency_nanos: i64::try_from(d.as_nanos()).unwrap_or(i64::MAX),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr as _;
    use std::time::Duration;

    use prost::Message as _;

    use crate::cli::{Args, HttpProtocol};
    use crate::proto::LoadTestRunReport;
    use crate::report::build_run_report;
    use crate::report_fixtures::{
        expected_cdf, expected_empty, expected_epoch, expected_full, expected_multi_worker,
        expected_no_headers, expected_zero_rps,
    };
    use crate::stats::WorkerStats;
    use crate::worker_manager::RunResult;

    fn minimal_args(url: &str, method: &str, rps: u32, duration_secs: u64) -> Args {
        let mut header = http::HeaderMap::new();
        header.insert("X-Foo", "Bar".parse().unwrap());
        args_with_headers(url, method, rps, duration_secs, header)
    }

    fn args_with_headers(
        url: &str,
        method: &str,
        rps: u32,
        duration_secs: u64,
        header: http::HeaderMap,
    ) -> Args {
        Args {
            url: url::Url::parse(url).unwrap(),
            header,
            insecure: false,
            request: http::Method::from_str(method).unwrap(),
            cacert: None,
            cert: None,
            key: None,
            location: false,
            requests_per_second: rust_decimal::Decimal::from(rps),
            duration: Duration::from_secs(duration_secs),
            max_time: None,
            connect_timeout: None,
            output: PathBuf::from("report.pb"),
            protocol: HttpProtocol::Http1_1,
            payload: None,
            identity: None,
            root_certificates: None,
            dry_run: true,
            upload_file_path: None,
        }
    }

    fn run_result_full() -> RunResult {
        use std::time::UNIX_EPOCH;
        let run_timestamp = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        RunResult {
            run_timestamp,
            worker_stats: vec![WorkerStats {
                timestamp: run_timestamp,
                id: 0,
                elapsed: Duration::from_millis(5000),
                request_sent: 100,
                in_flight: 0,
                informational_response: 0,
                successful_response: 95,
                redirection_message: 0,
                client_error_response: 2,
                server_error_response: 1,
                other_error_response: 2,
                timeouts: 0,
            }],
            success_latencies: vec![
                Duration::from_millis(10),
                Duration::from_millis(20),
                Duration::from_millis(30),
            ],
            non_success_latencies: vec![Duration::from_millis(100)],
        }
    }

    fn run_result_empty() -> RunResult {
        use std::time::UNIX_EPOCH;
        RunResult {
            run_timestamp: UNIX_EPOCH + Duration::from_secs(1000),
            worker_stats: vec![],
            success_latencies: vec![],
            non_success_latencies: vec![],
        }
    }

    fn run_result_multi_worker() -> RunResult {
        use std::time::UNIX_EPOCH;
        let run_timestamp = UNIX_EPOCH + Duration::from_secs(2000);
        RunResult {
            run_timestamp,
            worker_stats: vec![
                WorkerStats {
                    timestamp: run_timestamp,
                    id: 0,
                    elapsed: Duration::from_secs(60),
                    request_sent: 100,
                    in_flight: 0,
                    informational_response: 0,
                    successful_response: 80,
                    redirection_message: 0,
                    client_error_response: 10,
                    server_error_response: 5,
                    other_error_response: 5,
                    timeouts: 0,
                },
                WorkerStats {
                    timestamp: run_timestamp,
                    id: 1,
                    elapsed: Duration::from_secs(60),
                    request_sent: 120,
                    in_flight: 2,
                    informational_response: 0,
                    successful_response: 100,
                    redirection_message: 0,
                    client_error_response: 15,
                    server_error_response: 2,
                    other_error_response: 3,
                    timeouts: 0,
                },
            ],
            success_latencies: vec![Duration::from_millis(50)],
            non_success_latencies: vec![],
        }
    }

    fn run_result_cdf() -> RunResult {
        use std::time::UNIX_EPOCH;
        RunResult {
            run_timestamp: UNIX_EPOCH + Duration::from_secs(0),
            worker_stats: vec![],
            success_latencies: vec![
                Duration::from_millis(10),
                Duration::from_millis(20),
                Duration::from_millis(30),
            ],
            non_success_latencies: vec![Duration::from_millis(100)],
        }
    }

    fn run_result_no_headers() -> RunResult {
        use std::time::UNIX_EPOCH;
        RunResult {
            run_timestamp: UNIX_EPOCH + Duration::from_secs(1),
            worker_stats: vec![],
            success_latencies: vec![Duration::from_millis(1)],
            non_success_latencies: vec![],
        }
    }

    fn run_result_epoch() -> RunResult {
        use std::time::UNIX_EPOCH;
        RunResult {
            run_timestamp: UNIX_EPOCH,
            worker_stats: vec![],
            success_latencies: vec![Duration::from_nanos(1)],
            non_success_latencies: vec![],
        }
    }

    fn run_result_zero_rps() -> RunResult {
        use std::time::UNIX_EPOCH;
        RunResult {
            run_timestamp: UNIX_EPOCH + Duration::from_secs(1),
            worker_stats: vec![],
            success_latencies: vec![Duration::from_millis(1)],
            non_success_latencies: vec![],
        }
    }

    #[test]
    fn build_run_report_returns_serialized_protobuf_with_config_and_cdfs() {
        let args = minimal_args("https://test.example/", "GET", 10, 5);
        let result = run_result_full();
        let bytes = build_run_report(&args, &result).unwrap();
        let report = LoadTestRunReport::decode(bytes.as_slice()).unwrap();
        assert_eq!(report, expected_full());
    }

    #[test]
    fn build_run_report_empty_latencies_and_workers() {
        let args = minimal_args("https://empty.example/", "GET", 5, 10);
        let result = run_result_empty();
        let bytes = build_run_report(&args, &result).unwrap();
        let report = LoadTestRunReport::decode(bytes.as_slice()).unwrap();
        assert_eq!(report, expected_empty());
    }

    #[test]
    fn build_run_report_multiple_workers() {
        let args = minimal_args("https://multi.example/", "POST", 20, 60);
        let result = run_result_multi_worker();
        let bytes = build_run_report(&args, &result).unwrap();
        let report = LoadTestRunReport::decode(bytes.as_slice()).unwrap();
        assert_eq!(report, expected_multi_worker());
    }

    #[test]
    fn build_run_report_cdf_latency_values() {
        let args = minimal_args("https://cdf.example/", "GET", 1, 1);
        let result = run_result_cdf();
        let bytes = build_run_report(&args, &result).unwrap();
        let report = LoadTestRunReport::decode(bytes.as_slice()).unwrap();
        assert_eq!(report, expected_cdf());
    }

    #[test]
    fn build_run_report_config_no_headers() {
        let args = args_with_headers(
            "https://noheaders.example/",
            "GET",
            1,
            1,
            http::HeaderMap::new(),
        );
        let result = run_result_no_headers();
        let bytes = build_run_report(&args, &result).unwrap();
        let report = LoadTestRunReport::decode(bytes.as_slice()).unwrap();
        assert_eq!(report, expected_no_headers());
    }

    #[test]
    fn build_run_report_timestamp_at_epoch() {
        let args = minimal_args("https://epoch.example/", "GET", 1, 1);
        let result = run_result_epoch();
        let bytes = build_run_report(&args, &result).unwrap();
        let report = LoadTestRunReport::decode(bytes.as_slice()).unwrap();
        assert_eq!(report, expected_epoch());
    }

    #[test]
    fn build_run_report_config_method_and_zero_rps() {
        let args = minimal_args("https://config.example/", "POST", 0, 0);
        let result = run_result_zero_rps();
        let bytes = build_run_report(&args, &result).unwrap();
        let report = LoadTestRunReport::decode(bytes.as_slice()).unwrap();
        assert_eq!(report, expected_zero_rps());
    }
}
