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
    args_from_cli(cli).await
}

async fn args_from_cli(cli: Cli) -> Result<Args> {
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
pub(crate) fn parse_header_line(s: &str) -> Result<(http::HeaderName, http::HeaderValue)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<std::ffi::OsString> {
        std::iter::once(std::ffi::OsString::from("loadtest"))
            .chain(args.iter().map(|s| std::ffi::OsString::from(*s)))
            .collect::<Vec<_>>()
    }

    /// Parse from an explicit list of arguments. First element should be the program name.
    async fn parse_from<I, T>(args: I) -> Result<Args>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let cli = Cli::try_parse_from(args).map_err(AppError::from)?;
        args_from_cli(cli).await
    }

    /// Table-driven tests: parse_from(argv) succeeds; use `|args| { ... }` so the binding is in scope.
    macro_rules! parse_from_ok_tests {
        ($($name:ident: $argv:expr => |$args:ident| $body:block),* $(,)?) => {
            $(
                #[tokio::test]
                async fn $name() {
                    let $args = parse_from(argv($argv)).await.unwrap();
                    $body
                }
            )*
        };
    }

    /// Table-driven tests: parse_from(argv) fails and the error matches the given pattern.
    macro_rules! parse_from_err_tests {
        ($($name:ident: $argv:expr => $err_pat:pat),* $(,)?) => {
            $(
                #[tokio::test]
                async fn $name() {
                    let err = parse_from(argv($argv)).await.unwrap_err();
                    assert!(matches!(err, $err_pat));
                }
            )*
        };
    }

    parse_from_ok_tests! {
        parse_minimal_http1: &["https://example.com/", "--http1.1"] => |args| {
            assert_eq!(args.url.as_str(), "https://example.com/");
            assert!(matches!(args.protocol, HttpProtocol::Http1_1));
            assert_eq!(args.request, http::Method::GET);
            assert!(!args.insecure);
            assert!(!args.location);
            assert_eq!(args.requests_per_second, rust_decimal::Decimal::from(1));
            assert_eq!(args.duration, Duration::from_secs(10));
            assert!(args.max_time.is_none());
            assert!(args.connect_timeout.is_none());
            assert!(args.payload.is_none());
            assert!(args.identity.is_none());
            assert!(args.root_certificates.is_none());
            assert!(!args.dry_run);
            assert!(args.output.as_os_str().to_str().unwrap().starts_with("loadtest-report-"));
            assert!(args.output.as_os_str().to_str().unwrap().ends_with(".pb"));
        },
        parse_http2_prior_knowledge: &["https://example.com/", "--http2-prior-knowledge"] => |args| {
            assert!(matches!(args.protocol, HttpProtocol::Http2));
        },
        parse_duration_and_rps: &[
            "https://example.com/",
            "--http1.1",
            "--duration",
            "5",
            "--requests-per-second",
            "10",
        ] => |args| {
            assert_eq!(args.duration, Duration::from_secs(5));
            assert_eq!(args.requests_per_second, rust_decimal::Decimal::from(10));
        },
        parse_headers: &[
            "https://example.com/",
            "--http1.1",
            "-H",
            "X-Foo: bar",
            "-H",
            "Accept: application/json",
        ] => |args| {
            assert_eq!(args.header.get("x-foo").and_then(|v| v.to_str().ok()), Some("bar"));
            assert_eq!(
                args.header.get("accept").and_then(|v| v.to_str().ok()),
                Some("application/json")
            );
        },
        parse_request_method: &["https://example.com/", "--http1.1", "-X", "POST"] => |args| {
            assert_eq!(args.request, http::Method::POST);
        },
        parse_insecure_and_location: &["https://example.com/", "--http1.1", "-k", "-L"] => |args| {
            assert!(args.insecure);
            assert!(args.location);
        },
        parse_output_path: &[
            "https://example.com/",
            "--http1.1",
            "-o",
            "/tmp/report.pb",
        ] => |args| {
            assert_eq!(args.output.as_path(), std::path::Path::new("/tmp/report.pb"));
        },
        parse_dry_run: &["https://example.com/", "--http1.1", "--dry-run"] => |args| {
            assert!(args.dry_run);
        },
        parse_data_payload: &[
            "https://example.com/",
            "--http1.1",
            "-d",
            "hello world",
        ] => |args| {
            let payload = args.payload.as_ref().unwrap();
            match payload {
                Payload::Data(b) => assert_eq!(b.as_ref(), b"hello world"),
                Payload::File(_) => panic!("expected Data payload"),
            }
            assert_eq!(args.request, http::Method::POST);
        },
        parse_max_time_and_connect_timeout: &[
            "https://example.com/",
            "--http1.1",
            "-m",
            "30",
            "--connect-timeout",
            "2.5",
        ] => |args| {
            assert_eq!(args.max_time, Some(Duration::from_secs(30)));
            assert_eq!(args.connect_timeout, Some(Duration::from_secs_f64(2.5)));
        },
        parse_cacert_dry_run: &[
            "https://example.com/",
            "--http1.1",
            "--dry-run",
            "--cacert",
            "/etc/ssl/cacert.pem",
        ] => |args| {
            assert!(args.dry_run);
            assert_eq!(
                args.cacert.as_deref(),
                Some(std::path::Path::new("/etc/ssl/cacert.pem"))
            );
        },
        parse_cert_and_key_paths_dry_run: &[
            "https://example.com/",
            "--http1.1",
            "--dry-run",
            "-E",
            "/path/to/cert.pem",
            "--key",
            "/path/to/key.pem",
        ] => |args| {
            assert!(args.dry_run);
            assert_eq!(
                args.cert.as_deref(),
                Some(std::path::Path::new("/path/to/cert.pem"))
            );
            assert_eq!(
                args.key.as_deref(),
                Some(std::path::Path::new("/path/to/key.pem"))
            );
            assert!(args.identity.is_none());
        },
        parse_upload_file_path_dry_run: &[
            "https://example.com/",
            "--http1.1",
            "--dry-run",
            "-T",
            "/tmp/upload.bin",
        ] => |args| {
            assert!(args.dry_run);
            assert_eq!(
                args.upload_file_path.as_deref(),
                Some(std::path::Path::new("/tmp/upload.bin"))
            );
        },
    }

    parse_from_err_tests! {
        parse_invalid_url_fails: &["not-a-url", "--http1.1"] => AppError::InvalidUrl { .. },
        parse_missing_protocol_fails: &["https://example.com/"] => AppError::SpecifyProtocol,
        parse_mutually_exclusive_protocols_fails: &[
            "https://example.com/",
            "--http1.1",
            "--http2-prior-knowledge",
        ] => AppError::MutuallyExclusiveProtocols,
        parse_mutually_exclusive_upload_file_and_data_fails: &[
            "https://example.com/",
            "--http1.1",
            "-d",
            "data",
            "-T",
            "/tmp/file",
        ] => AppError::MutuallyExclusiveUploadFileAndData,
        parse_cert_without_key_fails: &[
            "https://example.com/",
            "--http1.1",
            "-E",
            "/cert.pem",
        ] => AppError::KeyMustBeProvidedWithCert,
        parse_key_without_cert_fails: &[
            "https://example.com/",
            "--http1.1",
            "--key",
            "/key.pem",
        ] => AppError::CertMustBeProvidedWithKey,
    }

    /// Fails to compile if a new field is added to `Args` without updating this test (and the coverage matrix).
    #[tokio::test]
    async fn args_exhaustive_destructure() {
        let args = parse_from(argv(&["https://example.com/", "--http1.1"]))
            .await
            .unwrap();
        let Args {
            url,
            header,
            insecure,
            request,
            cacert,
            cert,
            key,
            location,
            requests_per_second,
            duration,
            max_time,
            connect_timeout,
            output,
            protocol,
            payload,
            identity,
            root_certificates,
            dry_run,
            upload_file_path,
        } = &args;
        assert!(!url.as_str().is_empty());
        assert!(header.is_empty());
        assert!(!*insecure);
        assert_eq!(request.as_str(), "GET");
        assert!(cacert.is_none());
        assert!(cert.is_none());
        assert!(key.is_none());
        assert!(!*location);
        assert_eq!(requests_per_second, &rust_decimal::Decimal::from(1));
        assert_eq!(duration, &Duration::from_secs(10));
        assert!(max_time.is_none());
        assert!(connect_timeout.is_none());
        assert!(!output.as_os_str().is_empty());
        assert!(matches!(protocol, HttpProtocol::Http1_1));
        assert!(payload.is_none());
        assert!(identity.is_none());
        assert!(root_certificates.is_none());
        assert!(!*dry_run);
        assert!(upload_file_path.is_none());
    }

    #[tokio::test]
    async fn parse_upload_file_payload() {
        let dir = std::env::temp_dir();
        let path = dir.join("loadtest-upload-file-payload-test.bin");
        let content = b"file body content";
        tokio::fs::write(&path, content).await.unwrap();
        let path_str = path.to_string_lossy();
        let args = parse_from(argv(&[
            "https://example.com/",
            "--http1.1",
            "-T",
            path_str.as_ref(),
        ]))
        .await
        .unwrap();
        let payload = args.payload.as_ref().unwrap();
        match payload {
            Payload::File(b) => assert_eq!(b.as_ref(), content),
            Payload::Data(_) => panic!("expected File payload"),
        }
        assert_eq!(args.request, http::Method::PUT);
        let _ = tokio::fs::remove_file(&path).await;
    }
}
