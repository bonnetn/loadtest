//! Crate error type with thiserror and miette for pretty diagnostics.

use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// Loadtest error.
#[non_exhaustive]
#[derive(Error, Diagnostic, Debug)]
pub enum AppError {
    /// Header line has no colon (expected "Name: Value").
    #[error("Invalid header {raw:?} expected \"Name: Value\" (missing colon)")]
    #[diagnostic(code(loadtest::invalid_header_format))]
    InvalidHeaderFormat { raw: String },

    /// Header name contains invalid characters.
    #[error("Invalid header name in \"{raw}\"")]
    #[diagnostic(code(loadtest::invalid_header_name))]
    InvalidHeaderName {
        raw: String,
        source: http::header::InvalidHeaderName,
    },

    /// Header value contains invalid characters.
    #[error("Invalid header value in \"{raw}\"")]
    #[diagnostic(code(loadtest::invalid_header_value))]
    InvalidHeaderValue {
        raw: String,
        source: http::header::InvalidHeaderValue,
    },

    /// URL could not be parsed.
    #[error("Invalid URL \"{raw}\"")]
    #[diagnostic(code(loadtest::invalid_url))]
    InvalidUrl {
        raw: String,
        source: url::ParseError,
    },

    #[error("Join error")]
    #[diagnostic(code(loadtest::join_error))]
    JoinError {
        #[from]
        source: tokio::task::JoinError,
    },

    #[error("Failed to build client")]
    #[diagnostic(code(loadtest::failed_to_build_client))]
    FailedToBuildClient { source: reqwest::Error },

    #[error("Failed to send request")]
    #[diagnostic(code(loadtest::failed_to_send_request))]
    FailedToSendRequest { source: reqwest::Error },

    #[error("Failed to get response text")]
    #[diagnostic(code(loadtest::failed_to_get_response_text))]
    FailedToGetResponseChunk { source: reqwest::Error },

    #[error("I/O error: {0}")]
    #[diagnostic(code(loadtest::io))]
    Io(#[from] std::io::Error),

    #[error("Failed to encode protobuf report: {0}")]
    #[diagnostic(code(loadtest::proto_encode))]
    ProtoEncode(#[from] prost::EncodeError),

    #[error("http1.1 and http2-prior-knowledge are mutually exclusive")]
    #[diagnostic(code(loadtest::mutually_exclusive_protocols))]
    MutuallyExclusiveProtocols,

    #[error("Specify either --http1.1 or --http2-prior-knowledge")]
    #[diagnostic(code(loadtest::specify_protocol))]
    SpecifyProtocol,

    #[error("upload-file and data are mutually exclusive")]
    #[diagnostic(code(loadtest::mutually_exclusive_upload_file_and_data))]
    MutuallyExclusiveUploadFileAndData,

    #[error("Failed to open file {path:?}")]
    #[diagnostic(code(loadtest::failed_to_open_file))]
    FailedToOpenFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to read file {path:?}")]
    #[diagnostic(code(loadtest::failed_to_read_file))]
    FailedToReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("If --key is provided, --cert must also be provided")]
    #[diagnostic(code(loadtest::cert_must_be_provided_with_key))]
    CertMustBeProvidedWithKey,

    #[error("If --cert is provided, --key must also be provided")]
    #[diagnostic(code(loadtest::key_must_be_provided_with_cert))]
    KeyMustBeProvidedWithCert,

    #[error("Failed to parse identity PEM file")]
    #[diagnostic(code(loadtest::failed_to_parse_identity_pem_file))]
    FailedToParseIdentityPemFile { source: reqwest::Error },

    #[error("Failed to parse CA certificate PEM file")]
    #[diagnostic(code(loadtest::failed_to_parse_cacert_pem_file))]
    FailedToParseCacertPemFile { source: reqwest::Error },

    /// Invalid or missing CLI arguments.
    #[error(transparent)]
    #[diagnostic(code(loadtest::clap))]
    Clap(#[from] clap::Error),
}
