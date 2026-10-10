use futures::StreamExt as _;
use std::time::{Duration, Instant};

use cyper::Client;

use crate::{
    cli::{Args, HttpProtocol, Payload},
    error::{AppError, Result},
};

pub trait WorkUnit {
    fn execute(&self) -> impl Future<Output = Result<ExecutionResult>> + '_;
}

#[derive(Clone)]
pub struct RequestWorkUnit {
    args: Args,
    client: Client,
}

impl WorkUnit for RequestWorkUnit {
    fn execute(&self) -> impl Future<Output = Result<ExecutionResult>> + '_ {
        self.do_execute()
    }
}

impl RequestWorkUnit {
    pub fn new(args: &Args) -> Result<Self> {
        let mut builder = Client::builder()
            .default_headers(args.header.clone())
            .use_rustls_default()
            .danger_accept_invalid_certs(args.insecure);

        if matches!(args.protocol, HttpProtocol::Http2) {
            builder = builder.http2_prior_knowledge();
        }

        builder = if args.location {
            builder.redirect(cyper::redirect::Policy::limited(50))
        } else {
            builder.redirect(cyper::redirect::Policy::none())
        };

        let client = builder
            .build()
            .map_err(|source| AppError::FailedToBuildClient { source })?;

        Ok(Self {
            args: args.clone(),
            client,
        })
    }

    async fn do_execute(&self) -> Result<ExecutionResult> {
        let start = Instant::now();
        let future = self.do_execute_inner();
        let outcome = if let Some(timeout) = self.args.max_time {
            match compio::time::timeout(timeout, future).await {
                Ok(result) => result?,
                Err(_) => HttpRequestOutcome::Timeout,
            }
        } else {
            future.await?
        };
        Ok(ExecutionResult {
            outcome,
            duration: start.elapsed(),
        })
    }

    async fn do_execute_inner(&self) -> Result<HttpRequestOutcome> {
        let mut request = self
            .client
            .request(self.args.request.clone(), self.args.url.clone())
            .map_err(|source| AppError::FailedToSendRequest { source })?;
        if let Some(payload) = &self.args.payload {
            request = match payload {
                Payload::Data(data) | Payload::File(data) => request.body(data.clone()),
            };
        }

        let response = request
            .send()
            .await
            .map_err(|source| AppError::FailedToSendRequest { source })?;
        let status = response.status();
        let mut bytes_stream = response.bytes_stream();
        while let Some(result) = bytes_stream.next().await {
            result.map_err(|source| AppError::FailedToGetResponseChunk { source })?;
        }

        if status.is_informational() {
            Ok(HttpRequestOutcome::InformationalResponse)
        } else if status.is_redirection() {
            Ok(HttpRequestOutcome::RedirectionMessage)
        } else if status.is_success() {
            Ok(HttpRequestOutcome::SuccessResponse)
        } else if status.is_client_error() {
            Ok(HttpRequestOutcome::ClientErrorResponse)
        } else if status.is_server_error() {
            Ok(HttpRequestOutcome::ServerErrorResponse)
        } else {
            Ok(HttpRequestOutcome::OtherError)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    pub outcome: HttpRequestOutcome,
    pub duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpRequestOutcome {
    SuccessResponse,
    InformationalResponse,
    RedirectionMessage,
    ClientErrorResponse,
    ServerErrorResponse,
    OtherError,
    Timeout,
}
