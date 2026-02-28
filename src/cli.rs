//! CLI argument parsing. Uses clap internally and exposes a simple struct.

use bytes::Bytes;
use clap::Parser;
use clap::builder::styling::{AnsiColor, Styles};
use clap::value_parser;
use reqwest::Identity;
use reqwest::tls::Certificate;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::time::Duration;
use tokio::fs::{self, File};
use tokio::io::AsyncReadExt as _;

use crate::error::{AppError, Result};

/// Colored help styling (clap v4 best practice: explicit styles for help/errors).
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Cyan.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Green.on_default().bold())
    .placeholder(AnsiColor::Blue.on_default())
    .error(AnsiColor::Red.on_default().bold())
    .valid(AnsiColor::Green.on_default())
    .invalid(AnsiColor::Yellow.on_default().bold())
    .context(AnsiColor::Cyan.on_default().dimmed())
    .context_value(AnsiColor::Cyan.on_default().italic());

/// Parsed CLI arguments (no clap dependency at the type level).
#[derive(Debug, Clone)]
pub struct Args {
    pub url: url::Url,
    pub header: http::HeaderMap,
    pub insecure: bool,
    pub request: http::Method,
    pub cacert: Option<PathBuf>,
    pub cert: Option<PathBuf>,
    pub key: Option<PathBuf>,
    pub location: bool,
    pub requests_per_second: rust_decimal::Decimal,
    pub duration: Duration,
    /// Per-request timeout (e.g. curl's -m/--max-time).
    pub max_time: Option<Duration>,
    /// Connection timeout (seconds, decimal allowed).
    pub connect_timeout: Option<Duration>,
    /// Path to write the protobuf run report.
    pub output: PathBuf,
    pub protocol: HttpProtocol,
    pub payload: Option<Payload>,
    pub identity: Option<Identity>,
    /// CA certificate(s) loaded from --cacert for use with `tls_certs_only`.
    pub root_certificates: Option<Vec<Certificate>>,
    /// If true, print config and exit without making requests.
    pub dry_run: bool,
    /// Path passed to -T/--upload-file (for display when `dry_run` and file not read).
    pub upload_file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum HttpProtocol {
    Http1_1,
    Http2,
}

impl AsRef<str> for HttpProtocol {
    fn as_ref(&self) -> &str {
        match *self {
            HttpProtocol::Http1_1 => "HTTP/1.1",
            HttpProtocol::Http2 => "HTTP/2",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Payload {
    Data(Bytes),
    File(Bytes),
}

/// Protocol selection flags (mutually exclusive).
#[derive(Parser, Debug, Default)]
struct CliProtocol {
    /// Use HTTP/1.1. This is mutually exclusive with --http2-prior-knowledge.
    #[arg(long = "http1.1")]
    http1_1: bool,

    /// Use HTTP/2 prior knowledge. This is mutually exclusive with --http1.1.
    #[arg(long = "http2-prior-knowledge")]
    http2_prior_knowledge: bool,
}

/// Load test CLI - HTTP client with curl-like options.
#[derive(Parser, Debug)]
#[command(name = "loadtest")]
#[command(about = "HTTP client with curl-like options")]
#[command(version)]
#[command(styles = STYLES)]
#[command(next_line_help = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// URL to request (e.g. <https://example.com>).
    #[arg(required = true)]
    url: String,

    /// Extra header(s) in "Name: Value" form (can be repeated).
    #[arg(short = 'H', long = "header", value_name = "NAME: VALUE")]
    header: Vec<String>,

    /// Disable TLS/SSL certificate verification.
    #[arg(short = 'k', long = "insecure")]
    insecure: bool,

    /// File to upload (path), note that the file is fully loaded into memory.
    #[arg(short = 'T', long = "upload-file", value_name = "PATH")]
    upload_file: Option<PathBuf>,

    /// HTTP method (GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, PATCH).
    #[arg(short = 'X', long = "request", value_name = "METHOD", value_parser = http::Method::from_str)]
    request: Option<http::Method>,

    /// CA certificate file to use for peer verification.
    #[arg(long = "cacert", value_name = "FILE")]
    cacert: Option<PathBuf>,

    /// Path to client certificate file.
    #[arg(short = 'E', long = "cert", value_name = "FILE")]
    cert: Option<PathBuf>,

    /// Path to client key file.
    #[arg(long = "key", value_name = "FILE")]
    key: Option<PathBuf>,

    /// Raw data to send in the request body.
    #[arg(short = 'd', long = "data", value_name = "DATA")]
    data: Option<String>,

    /// Follow redirects (HTTP 3xx).
    #[arg(short = 'L', long = "location")]
    location: bool,

    /// Target requests per second (rate limit).
    #[arg(long = "requests-per-second", value_name = "RPS", default_value = "1", value_parser = value_parser!(rust_decimal::Decimal))]
    requests_per_second: rust_decimal::Decimal,

    /// Test duration in seconds.
    #[arg(long = "duration", value_name = "SECS", default_value = "10", value_parser = value_parser!(u64).range(1..))]
    duration_secs: u64,

    /// Maximum time to run the test (seconds).
    #[arg(short = 'm', long = "max-time", value_name = "SECONDS", value_parser = value_parser!(u64).range(1..))]
    max_time_secs: Option<u64>,

    /// Connection timeout in seconds (decimal allowed, e.g. 2.5).
    #[arg(long = "connect-timeout", value_name = "SECONDS", value_parser = value_parser!(f64))]
    connect_timeout_secs: Option<f64>,

    /// Write protobuf run report to this path (default: loadtest-report-YYYYMMDDTHHMMSS.pb).
    #[arg(short = 'o', long = "output", value_name = "PATH")]
    output: Option<PathBuf>,

    #[command(flatten)]
    protocol: CliProtocol,

    /// Print config and exit without making requests (skips reading --cacert, -T, --cert/--key).
    #[arg(long = "dry-run")]
    dry_run: bool,
}

/// Parse command-line arguments and return a simple struct.
pub async fn parse() -> Result<Args> {
    let cli = Cli::parse();
    let header = parse_headers(&cli.header)?;
    let url = url::Url::parse(&cli.url).map_err(|source| AppError::InvalidUrl {
        raw: cli.url.clone(),
        source,
    })?;
    let protocol = resolve_protocol(&cli.protocol)?;

    if cli.upload_file.is_some() && cli.data.is_some() {
        return Err(AppError::MutuallyExclusiveUploadFileAndData);
    }

    let (payload, upload_file_path) = resolve_payload_and_path(&cli).await?;
    let request = resolve_request(&cli, payload.as_ref(), upload_file_path.as_ref());
    let identity = resolve_identity(&cli).await?;
    let root_certificates = resolve_root_certificates(&cli).await?;

    Ok(Args {
        url,
        header,
        insecure: cli.insecure,
        request,
        cacert: cli.cacert,
        cert: cli.cert,
        key: cli.key,
        location: cli.location,
        requests_per_second: cli.requests_per_second,
        duration: Duration::from_secs(cli.duration_secs),
        max_time: cli.max_time_secs.map(Duration::from_secs),
        connect_timeout: cli.connect_timeout_secs.map(Duration::from_secs_f64),
        output: cli.output.unwrap_or_else(|| {
            let ts = chrono::Utc::now().format("%Y%m%dT%H%M%S");
            PathBuf::from(format!("loadtest-report-{ts}.pb"))
        }),
        protocol,
        payload,
        identity,
        root_certificates,
        dry_run: cli.dry_run,
        upload_file_path,
    })
}

fn parse_headers(header_strs: &[String]) -> Result<http::HeaderMap> {
    let mut header = http::HeaderMap::new();
    for s in header_strs {
        let (name, value) = parse_header_line(s)?;
        header.append(name, value);
    }
    Ok(header)
}

fn resolve_protocol(protocol: &CliProtocol) -> Result<HttpProtocol> {
    if protocol.http1_1 && protocol.http2_prior_knowledge {
        return Err(AppError::MutuallyExclusiveProtocols);
    }
    match (protocol.http1_1, protocol.http2_prior_knowledge) {
        (true, false) => Ok(HttpProtocol::Http1_1),
        (false, true) => Ok(HttpProtocol::Http2),
        (true, true) => Err(AppError::MutuallyExclusiveProtocols),
        (false, false) => Err(AppError::SpecifyProtocol),
    }
}

async fn resolve_payload_and_path(cli: &Cli) -> Result<(Option<Payload>, Option<PathBuf>)> {
    if cli.dry_run {
        let path_only = cli.upload_file.clone();
        let payload = match (cli.upload_file.as_ref(), cli.data.as_ref()) {
            (_, Some(data)) => Some(Payload::Data(Bytes::from(data.clone()))),
            (Some(_) | None, None) => None,
        };
        Ok((payload, path_only))
    } else {
        let payload = match (cli.upload_file.as_ref(), cli.data.as_ref()) {
            (Some(path), None) => {
                let mut file =
                    File::open(path)
                        .await
                        .map_err(|source| AppError::FailedToOpenFile {
                            path: path.clone(),
                            source,
                        })?;
                let mut content = Vec::new();
                file.read_to_end(&mut content).await.map_err(|source| {
                    AppError::FailedToReadFile {
                        path: path.clone(),
                        source,
                    }
                })?;
                Some(Payload::File(Bytes::from(content)))
            }
            (None, Some(data)) => Some(Payload::Data(Bytes::from(data.clone()))),
            (Some(_), Some(_)) => return Err(AppError::MutuallyExclusiveUploadFileAndData),
            (None, None) => None,
        };
        Ok((payload, None))
    }
}

fn resolve_request(
    cli: &Cli,
    payload: Option<&Payload>,
    upload_file_path: Option<&PathBuf>,
) -> http::Method {
    if let Some(ref request) = cli.request {
        request.clone()
    } else {
        match payload {
            Some(&Payload::Data(_)) => http::Method::POST,
            Some(&Payload::File(_)) => http::Method::PUT,
            None => {
                if upload_file_path.is_some() {
                    http::Method::PUT
                } else {
                    http::Method::GET
                }
            }
        }
    }
}

async fn resolve_identity(cli: &Cli) -> Result<Option<Identity>> {
    if cli.dry_run {
        return Ok(None);
    }
    match (cli.cert.as_ref(), cli.key.as_ref()) {
        (Some(cert), Some(key)) => {
            let cert_data = fs::read(cert)
                .await
                .map_err(|source| AppError::FailedToReadFile {
                    path: cert.clone(),
                    source,
                })?;
            let key_data = fs::read(key)
                .await
                .map_err(|source| AppError::FailedToReadFile {
                    path: key.clone(),
                    source,
                })?;
            let concat_data = [cert_data, key_data].concat();
            let identity = Identity::from_pem(&concat_data)
                .map_err(|source| AppError::FailedToParseIdentityPemFile { source })?;
            Ok(Some(identity))
        }
        (None, None) => Ok(None),
        (Some(_), None) => Err(AppError::KeyMustBeProvidedWithCert),
        (None, Some(_)) => Err(AppError::CertMustBeProvidedWithKey),
    }
}

async fn resolve_root_certificates(cli: &Cli) -> Result<Option<Vec<Certificate>>> {
    if cli.dry_run {
        return Ok(None);
    }
    if let Some(ref path) = cli.cacert {
        let data = fs::read(path)
            .await
            .map_err(|source| AppError::FailedToReadFile {
                path: path.clone(),
                source,
            })?;
        let certs = Certificate::from_pem_bundle(&data)
            .map_err(|source| AppError::FailedToParseCacertPemFile { source })?;
        Ok(Some(certs))
    } else {
        Ok(None)
    }
}

/// Parses a "Name: Value" header line. Splits on the first colon; name and value are trimmed.
fn parse_header_line(s: &str) -> Result<(http::HeaderName, http::HeaderValue)> {
    let s = s.trim();
    let Some((name, value)) = s.split_once(':') else {
        return Err(AppError::InvalidHeaderFormat { raw: s.to_owned() });
    };
    let name = name.trim();
    let value = value.trim();
    let name = http::HeaderName::from_str(name).map_err(|source| AppError::InvalidHeaderName {
        raw: s.to_owned(),
        source,
    })?;
    let value =
        http::HeaderValue::from_str(value).map_err(|source| AppError::InvalidHeaderValue {
            raw: s.to_owned(),
            source,
        })?;
    Ok((name, value))
}
