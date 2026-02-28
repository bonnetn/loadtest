use futures::StreamExt as _;

use crate::{
    cli::{Args, HttpProtocol, Payload},
    error::{AppError, Result},
};
use std::{
    future::Future,
    time::{Duration, Instant},
};

pub trait WorkUnit {
    fn execute(&self) -> impl Future<Output = Result<ExecutionResult>> + Send + '_;
}

pub struct RequestWorkUnit {
    args: Args,
    client: reqwest::Client,
}

impl WorkUnit for RequestWorkUnit {
    fn execute(&self) -> impl Future<Output = Result<ExecutionResult>> + Send + '_ {
        self.do_execute()
    }
}

impl RequestWorkUnit {
    pub fn new(args: &Args) -> Result<Self> {
        let mut builder = reqwest::Client::builder();
        if let Some(t) = args.max_time {
            builder = builder.timeout(t);
        }
        if let Some(t) = args.connect_timeout {
            builder = builder.connect_timeout(t);
        }

        match args.protocol {
            HttpProtocol::Http1_1 => {
                builder = builder.http1_only();
            }
            HttpProtocol::Http2 => {
                builder = builder.http2_prior_knowledge();
            }
        }

        if args.insecure {
            builder = builder.tls_danger_accept_invalid_certs(true);
            builder = builder.tls_danger_accept_invalid_hostnames(true);
        }

        builder = builder.default_headers(args.header.clone());

        if args.location {
            builder = builder.redirect(reqwest::redirect::Policy::limited(50));
        } else {
            builder = builder.redirect(reqwest::redirect::Policy::none());
        }

        if let Some(t) = args.max_time {
            builder = builder.timeout(t);
        }

        if let Some(ref identity) = args.identity {
            builder = builder.identity(identity.clone());
        }

        if let Some(ref certs) = args.root_certificates {
            builder = builder.tls_certs_only(certs.clone());
        }

        // Disable auto decompression of response bodies.
        builder = builder.no_brotli();
        builder = builder.no_deflate();
        builder = builder.no_gzip();
        builder = builder.no_zstd();

        // Use rustls
        builder = builder.tls_backend_rustls();

        // TODO:
        // builder = builder.http2_adaptive_window(true);
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
        let outcome = self.do_execute_inner().await?;
        let duration = start.elapsed();
        Ok(ExecutionResult { outcome, duration })
    }

    async fn do_execute_inner(&self) -> Result<HttpRequestOutcome> {
        let url = self.args.url.clone();
        let method = self.args.request.clone();

        let mut request_builder = self.client.request(method, url);
        match self.args.payload {
            Some(Payload::Data(ref data)) => {
                request_builder = request_builder.body(data.clone());
            }
            Some(Payload::File(ref file)) => {
                request_builder = request_builder.body(file.clone());
            }
            None => {}
        }

        let result = request_builder.send().await;

        if let Err(ref source) = result
            && source.is_timeout()
        {
            return Ok(HttpRequestOutcome::Timeout);
        }

        let response = result.map_err(|source| AppError::FailedToSendRequest { source })?;

        let status = response.status();
        let _headers = response.headers().clone();

        let mut bytes_stream = response.bytes_stream();
        while let Some(result) = bytes_stream.next().await {
            if let Err(ref source) = result
                && source.is_timeout()
            {
                return Ok(HttpRequestOutcome::Timeout);
            }

            let _chunk = result.map_err(|source| AppError::FailedToGetResponseChunk { source })?;
            // println!("chunk size : {:?}", chunk.len());
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
