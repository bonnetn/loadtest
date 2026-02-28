use std::time::{Duration, SystemTime};

use crate::work_unit::{ExecutionResult, HttpRequestOutcome};

#[derive(Debug, PartialEq, Eq)]
pub struct Statistics {
    pub informational_response: u64,
    pub successful_response: u64,
    pub redirection_message: u64,
    pub client_error_response: u64,
    pub server_error_response: u64,
    pub other_error_response: u64,
    pub timeouts: u64,
    pub success_latencies: Vec<Duration>,
    pub non_success_latencies: Vec<Duration>,
}

impl Statistics {
    pub fn new(capacity: usize) -> Self {
        Self {
            successful_response: 0,
            informational_response: 0,
            redirection_message: 0,
            client_error_response: 0,
            server_error_response: 0,
            other_error_response: 0,
            timeouts: 0,
            success_latencies: Vec::with_capacity(capacity),
            non_success_latencies: Vec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, result: &ExecutionResult) {
        let is_success = result.outcome == HttpRequestOutcome::SuccessResponse;
        match result.outcome {
            HttpRequestOutcome::InformationalResponse => {
                self.informational_response = self
                    .informational_response
                    .checked_add(1)
                    .expect("informational_response counter overflow");
            }
            HttpRequestOutcome::SuccessResponse => {
                self.successful_response = self
                    .successful_response
                    .checked_add(1)
                    .expect("successful_response counter overflow");
            }
            HttpRequestOutcome::RedirectionMessage => {
                self.redirection_message = self
                    .redirection_message
                    .checked_add(1)
                    .expect("redirection_message counter overflow");
            }
            HttpRequestOutcome::ClientErrorResponse => {
                self.client_error_response = self
                    .client_error_response
                    .checked_add(1)
                    .expect("client_error_response counter overflow");
            }
            HttpRequestOutcome::ServerErrorResponse => {
                self.server_error_response = self
                    .server_error_response
                    .checked_add(1)
                    .expect("server_error_response counter overflow");
            }
            HttpRequestOutcome::OtherError => {
                self.other_error_response = self
                    .other_error_response
                    .checked_add(1)
                    .expect("other_error_response counter overflow");
            }
            HttpRequestOutcome::Timeout => {
                self.timeouts = self.timeouts.checked_add(1).expect("timeouts counter overflow");
            }
        }
        if is_success {
            self.success_latencies.push(result.duration);
        } else {
            self.non_success_latencies.push(result.duration);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerStats {
    pub timestamp: SystemTime,
    pub id: usize,
    pub elapsed: Duration,
    pub request_sent: usize,
    pub in_flight: usize,
    pub informational_response: u64,
    pub successful_response: u64,
    pub redirection_message: u64,
    pub client_error_response: u64,
    pub server_error_response: u64,
    pub other_error_response: u64,
    pub timeouts: u64,
}
