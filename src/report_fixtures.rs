//! Hardcoded expected report structs for report tests.
#![cfg(test)]

use crate::proto::{
    CdfPoint, Header, LoadTestConfig, LoadTestRunReport, WorkerStats as ProtoWorkerStats,
};

pub(crate) fn expected_full() -> LoadTestRunReport {
    LoadTestRunReport {
        run_timestamp_unix_nanos: 1700000000000000000,
        config: Some(LoadTestConfig {
            url: "https://test.example/".into(),
            method: "GET".into(),
            requests_per_second: 10,
            duration_secs: 5,
            headers: vec![Header {
                name: "x-foo".into(),
                value: "Bar".into(),
            }],
        }),
        worker_stats: vec![ProtoWorkerStats {
            timestamp_unix_nanos: 1700000000000000000,
            worker_id: 0,
            elapsed_nanos: 5000000000,
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
        cdf: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 100000000,
            },
        ],
        cdf_success: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 30000000,
            },
        ],
        cdf_non_success: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 100000000,
            },
        ],
    }
}

pub(crate) fn expected_empty() -> LoadTestRunReport {
    LoadTestRunReport {
        run_timestamp_unix_nanos: 1000000000000,
        config: Some(LoadTestConfig {
            url: "https://empty.example/".into(),
            method: "GET".into(),
            requests_per_second: 5,
            duration_secs: 10,
            headers: vec![Header {
                name: "x-foo".into(),
                value: "Bar".into(),
            }],
        }),
        worker_stats: vec![],
        cdf: vec![],
        cdf_success: vec![],
        cdf_non_success: vec![],
    }
}

pub(crate) fn expected_multi_worker() -> LoadTestRunReport {
    LoadTestRunReport {
        run_timestamp_unix_nanos: 2000000000000,
        config: Some(LoadTestConfig {
            url: "https://multi.example/".into(),
            method: "POST".into(),
            requests_per_second: 20,
            duration_secs: 60,
            headers: vec![Header {
                name: "x-foo".into(),
                value: "Bar".into(),
            }],
        }),
        worker_stats: vec![
            ProtoWorkerStats {
                timestamp_unix_nanos: 2000000000000,
                worker_id: 0,
                elapsed_nanos: 60000000000,
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
            ProtoWorkerStats {
                timestamp_unix_nanos: 2000000000000,
                worker_id: 1,
                elapsed_nanos: 60000000000,
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
        cdf: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 50000000,
            },
        ],
        cdf_success: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 50000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 50000000,
            },
        ],
        cdf_non_success: vec![],
    }
}

pub(crate) fn expected_cdf() -> LoadTestRunReport {
    LoadTestRunReport {
        run_timestamp_unix_nanos: 0,
        config: Some(LoadTestConfig {
            url: "https://cdf.example/".into(),
            method: "GET".into(),
            requests_per_second: 1,
            duration_secs: 1,
            headers: vec![Header {
                name: "x-foo".into(),
                value: "Bar".into(),
            }],
        }),
        worker_stats: vec![],
        cdf: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 100000000,
            },
        ],
        cdf_success: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 10000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 20000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 30000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 30000000,
            },
        ],
        cdf_non_success: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 100000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 100000000,
            },
        ],
    }
}

pub(crate) fn expected_no_headers() -> LoadTestRunReport {
    LoadTestRunReport {
        run_timestamp_unix_nanos: 1000000000,
        config: Some(LoadTestConfig {
            url: "https://noheaders.example/".into(),
            method: "GET".into(),
            requests_per_second: 1,
            duration_secs: 1,
            headers: vec![],
        }),
        worker_stats: vec![],
        cdf: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 1000000,
            },
        ],
        cdf_success: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 1000000,
            },
        ],
        cdf_non_success: vec![],
    }
}

pub(crate) fn expected_epoch() -> LoadTestRunReport {
    LoadTestRunReport {
        run_timestamp_unix_nanos: 0,
        config: Some(LoadTestConfig {
            url: "https://epoch.example/".into(),
            method: "GET".into(),
            requests_per_second: 1,
            duration_secs: 1,
            headers: vec![Header {
                name: "x-foo".into(),
                value: "Bar".into(),
            }],
        }),
        worker_stats: vec![],
        cdf: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 1,
            },
        ],
        cdf_success: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 1,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 1,
            },
        ],
        cdf_non_success: vec![],
    }
}

pub(crate) fn expected_zero_rps() -> LoadTestRunReport {
    LoadTestRunReport {
        run_timestamp_unix_nanos: 1000000000,
        config: Some(LoadTestConfig {
            url: "https://config.example/".into(),
            method: "POST".into(),
            requests_per_second: 0,
            duration_secs: 0,
            headers: vec![Header {
                name: "x-foo".into(),
                value: "Bar".into(),
            }],
        }),
        worker_stats: vec![],
        cdf: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 1000000,
            },
        ],
        cdf_success: vec![
            CdfPoint {
                percentile: 0.0,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.13403567663993465,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.25010579066754424,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.3506183684237888,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.43765867480965104,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.513032474834137,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.5783034965714178,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.6348258727451623,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.6837722339831622,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.726158036573564,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.7628626294338345,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.7946474973542854,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8221720589961078,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8460073473940508,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8666478567836676,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.8845218015310542,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9134035676639936,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9250105790667544,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9350618368423789,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9437658674809651,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9513032474834137,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9578303496571418,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9634825872745163,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9683772233983162,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9726158036573564,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9762862629433835,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9794647497354285,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9822172058996108,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9846007347394051,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9866647856783668,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9884521801531054,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.99,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9913403567663993,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9925010579066754,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9935061836842379,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9943765867480965,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9951303247483414,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9957830349657142,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9963482587274516,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9968377223398316,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9972615803657356,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9976286262943383,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9979464749735428,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9982217205899611,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9984600734739405,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9986664785678366,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9988452180153106,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.999,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9991340356766399,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9992501057906675,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9993506183684238,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9994376586748096,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9995130324748341,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9995783034965714,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9996348258727452,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9996837722339832,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997261580365736,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997628626294338,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9997946474973542,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998221720589962,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.999846007347394,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998666478567837,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9998845218015311,
                latency_nanos: 1000000,
            },
            CdfPoint {
                percentile: 0.9999,
                latency_nanos: 1000000,
            },
        ],
        cdf_non_success: vec![],
    }
}
