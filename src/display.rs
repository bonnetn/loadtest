//! Format CLI args as human-readable text (for dry-run output and testing).

use crate::cli::{Args, Payload};

/// Returns the same content as the dry-run output, as plain text (no ANSI styling).
/// Callers can print this to stdout or assert on it in tests.
pub fn format_args(args: &Args) -> String {
    let mut out = String::new();
    out.push_str("Arguments\n");
    out.push_str(&format_args_request(args));
    out.push_str(&format_args_tls(args));
    out.push_str(&format_args_timing(args));
    out.push_str(&format_args_output(args));
    out
}

fn format_args_request(args: &Args) -> String {
    let mut out = String::new();
    out.push_str(&format!("  URL: {}\n", args.url));
    out.push_str("  Headers:\n");
    for (name, value) in &args.header {
        let value_str = value.to_str().unwrap_or("<invalid>");
        out.push_str(&format!("    {}: {}\n", name.as_str(), value_str));
    }
    out.push_str(&format!("  Insecure: {}\n", args.insecure));
    out.push_str(&format!("  Method: {}\n", args.request));
    out
}

fn format_args_tls(args: &Args) -> String {
    let mut out = String::new();
    if let Some(ref cacert) = args.cacert {
        out.push_str(&format!("  CA certificate file: {}\n", cacert.display()));
    }
    if let Some(ref cert) = args.cert {
        out.push_str(&format!("  Certificate file : {}\n", cert.display()));
    }
    if let Some(ref key) = args.key {
        out.push_str(&format!("  Key file: {}\n", key.display()));
    }
    out
}

fn format_args_timing(args: &Args) -> String {
    let mut out = String::new();
    out.push_str(&format!("  Follow redirects: {}\n", args.location));
    out.push_str(&format!(
        "  Throughput: {} requests/second\n",
        args.requests_per_second
    ));
    out.push_str(&format!(
        "  Load test duration: {} seconds\n",
        args.duration.as_secs()
    ));
    if let Some(max_time) = args.max_time {
        out.push_str(&format!(
            "  Request timeout: {} seconds\n",
            max_time.as_secs()
        ));
    }
    if let Some(connect_timeout) = args.connect_timeout {
        out.push_str(&format!(
            "  Connection timeout: {} seconds\n",
            connect_timeout.as_secs()
        ));
    }
    out
}

fn format_args_output(args: &Args) -> String {
    let mut out = String::new();
    out.push_str(&format!("  Output file: {}\n", args.output.display()));
    out.push_str(&format!("  Protocol: {}\n", args.protocol.as_ref()));
    let payload_size = match args.payload {
        Some(Payload::Data(ref data)) => data.len(),
        Some(Payload::File(ref file)) => file.len(),
        None => 0,
    };
    out.push_str(&format!("  Request body size: {} bytes\n", payload_size));
    if let Some(ref path) = args.upload_file_path {
        out.push_str(&format!("  Upload file: {}\n", path.display()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{Args, HttpProtocol, Payload};
    use bytes::Bytes;
    use std::path::PathBuf;
    use std::time::Duration;

    fn minimal_args(url: &str) -> Args {
        Args {
            url: url::Url::parse(url).unwrap(),
            header: http::HeaderMap::new(),
            insecure: false,
            request: http::Method::GET,
            cacert: None,
            cert: None,
            key: None,
            location: false,
            requests_per_second: rust_decimal::Decimal::from(1),
            duration: Duration::from_secs(10),
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

    /// Table-driven tests: each row is `test_name: args_expr => (assert_kind "string"), ...`
    /// Assert kinds: `contains`, `not_contains`, `starts_with`, `eq`.
    macro_rules! format_args_table {
        ($($name:ident: $args:expr => $(($assert:ident $s:expr)),+ $(,)?);+ $(;)?) => {
            $(
                #[test]
                fn $name() {
                    let args = $args;
                    let out = format_args(&args);
                    $(
                        format_args_assert!($assert, out, $s);
                    )+
                }
            )+
        };
    }

    macro_rules! format_args_assert {
        (contains, $out:expr, $s:expr) => {
            assert!(
                $out.contains($s),
                "expected output to contain {:?}, got: {:?}",
                $s,
                $out
            )
        };
        (not_contains, $out:expr, $s:expr) => {
            assert!(
                !$out.contains($s),
                "expected output not to contain {:?}, got: {:?}",
                $s,
                $out
            )
        };
        (starts_with, $out:expr, $s:expr) => {
            assert!(
                $out.starts_with($s),
                "expected output to start with {:?}, got: {:?}",
                $s,
                $out
            )
        };
        (eq, $out:expr, $s:expr) => {
            assert_eq!($out, $s, "output mismatch")
        };
    }

    const MINIMAL_OUTPUT: &str = concat!(
        "Arguments\n",
        "  URL: https://example.com/\n",
        "  Headers:\n",
        "  Insecure: false\n",
        "  Method: GET\n",
        "  Follow redirects: false\n",
        "  Throughput: 1 requests/second\n",
        "  Load test duration: 10 seconds\n",
        "  Output file: report.pb\n",
        "  Protocol: HTTP/1.1\n",
        "  Request body size: 0 bytes\n",
    );

    format_args_table! {
        format_args_starts_with_arguments_header: minimal_args("https://example.com") => (starts_with "Arguments\n");
        format_args_includes_url_and_method: minimal_args("https://example.com/path") => (contains "Arguments"), (contains "https://example.com/path"), (contains "URL:"), (contains "Method: GET");
        format_args_includes_all_section_headers: minimal_args("https://example.com") => (contains "  URL:"), (contains "  Headers:"), (contains "  Insecure:"), (contains "  Method:"), (contains "  Follow redirects:"), (contains "  Throughput:"), (contains "  Load test duration:"), (contains "  Output file:"), (contains "  Protocol:"), (contains "  Request body size:");

        format_args_request_url: minimal_args("https://user:pass@example.com:443/path?q=1") => (contains "  URL: "), (contains "example.com"), (contains "/path"), (contains "q=1");
        format_args_request_empty_headers: minimal_args("https://example.com") => (contains "  Headers:\n"), (starts_with "Arguments\n");
        format_args_includes_headers: {
            let mut a = minimal_args("https://example.com");
            a.header.append(http::header::ACCEPT, "application/json".parse().unwrap());
            a
        } => (contains "accept"), (contains "application/json");
        format_args_request_multiple_headers: {
            let mut a = minimal_args("https://example.com");
            a.header.append(http::header::ACCEPT, "application/json".parse().unwrap());
            a.header.append(http::header::CONTENT_TYPE, "text/plain".parse().unwrap());
            a
        } => (contains "accept"), (contains "application/json"), (contains "content-type"), (contains "text/plain");
        format_args_request_insecure_false: minimal_args("https://example.com") => (contains "  Insecure: false\n");
        format_args_request_insecure_true: { let mut a = minimal_args("https://example.com"); a.insecure = true; a } => (contains "  Insecure: true\n");
        format_args_request_method_get: minimal_args("https://example.com") => (contains "  Method: GET\n");
        format_args_request_method_post: { let mut a = minimal_args("https://example.com"); a.request = http::Method::POST; a } => (contains "  Method: POST\n");
        format_args_request_method_put: { let mut a = minimal_args("https://example.com"); a.request = http::Method::PUT; a } => (contains "  Method: PUT\n");
        format_args_request_method_delete: { let mut a = minimal_args("https://example.com"); a.request = http::Method::DELETE; a } => (contains "  Method: DELETE\n");
        format_args_request_method_patch: { let mut a = minimal_args("https://example.com"); a.request = http::Method::PATCH; a } => (contains "  Method: PATCH\n");

        format_args_tls_none: minimal_args("https://example.com") => (not_contains "CA certificate file:"), (not_contains "Certificate file"), (not_contains "Key file:");
        format_args_tls_cacert_only: { let mut a = minimal_args("https://example.com"); a.cacert = Some(PathBuf::from("/etc/ssl/ca.pem")); a } => (contains "  CA certificate file: /etc/ssl/ca.pem\n"), (not_contains "Certificate file"), (not_contains "Key file:");
        format_args_tls_cert_only: { let mut a = minimal_args("https://example.com"); a.cert = Some(PathBuf::from("/etc/ssl/client.pem")); a } => (contains "  Certificate file : /etc/ssl/client.pem\n"), (not_contains "CA certificate file:"), (not_contains "Key file:");
        format_args_tls_key_only: { let mut a = minimal_args("https://example.com"); a.key = Some(PathBuf::from("/etc/ssl/client.key")); a } => (contains "  Key file: /etc/ssl/client.key\n"), (not_contains "CA certificate file:"), (not_contains "Certificate file");
        format_args_tls_all_three: {
            let mut a = minimal_args("https://example.com");
            a.cacert = Some(PathBuf::from("ca.pem"));
            a.cert = Some(PathBuf::from("cert.pem"));
            a.key = Some(PathBuf::from("key.pem"));
            a
        } => (contains "  CA certificate file: ca.pem\n"), (contains "  Certificate file : cert.pem\n"), (contains "  Key file: key.pem\n");

        format_args_timing_follow_redirects_false: minimal_args("https://example.com") => (contains "  Follow redirects: false\n");
        format_args_timing_follow_redirects_true: { let mut a = minimal_args("https://example.com"); a.location = true; a } => (contains "  Follow redirects: true\n");
        format_args_timing_throughput: { let mut a = minimal_args("https://example.com"); a.requests_per_second = rust_decimal::Decimal::from(100); a } => (contains "  Throughput: 100 requests/second\n");
        format_args_timing_throughput_decimal: { let mut a = minimal_args("https://example.com"); a.requests_per_second = rust_decimal::Decimal::try_from(0.5).unwrap(); a } => (contains "requests/second"), (contains "0.5");
        format_args_timing_duration: { let mut a = minimal_args("https://example.com"); a.duration = Duration::from_secs(60); a } => (contains "  Load test duration: 60 seconds\n");
        format_args_timing_max_time_none: minimal_args("https://example.com") => (not_contains "Request timeout:");
        format_args_timing_max_time_some: { let mut a = minimal_args("https://example.com"); a.max_time = Some(Duration::from_secs(30)); a } => (contains "  Request timeout: 30 seconds\n");
        format_args_timing_connect_timeout_none: minimal_args("https://example.com") => (not_contains "Connection timeout:");
        format_args_timing_connect_timeout_some: { let mut a = minimal_args("https://example.com"); a.connect_timeout = Some(Duration::from_secs(5)); a } => (contains "  Connection timeout: 5 seconds\n");

        format_args_output_file_path: { let mut a = minimal_args("https://example.com"); a.output = PathBuf::from("/var/report.pb"); a } => (contains "  Output file: /var/report.pb\n");
        format_args_output_protocol_http1_1: minimal_args("https://example.com") => (contains "  Protocol: HTTP/1.1\n");
        format_args_output_protocol_http2: { let mut a = minimal_args("https://example.com"); a.protocol = HttpProtocol::Http2; a } => (contains "  Protocol: HTTP/2\n");
        format_args_output_payload_none: minimal_args("https://example.com") => (contains "  Request body size: 0 bytes\n");
        format_args_output_payload_data: { let mut a = minimal_args("https://example.com"); a.payload = Some(Payload::Data(Bytes::from_static(b"hello"))); a } => (contains "  Request body size: 5 bytes\n");
        format_args_output_payload_file: { let mut a = minimal_args("https://example.com"); a.payload = Some(Payload::File(Bytes::from_static(b"file contents here"))); a } => (contains "  Request body size: 18 bytes\n");
        format_args_output_upload_file_path_none: minimal_args("https://example.com") => (not_contains "Upload file:");
        format_args_output_upload_file_path_some: { let mut a = minimal_args("https://example.com"); a.upload_file_path = Some(PathBuf::from("/tmp/upload.bin")); a } => (contains "  Upload file: /tmp/upload.bin\n");

        format_args_full_output_minimal: minimal_args("https://example.com") => (eq MINIMAL_OUTPUT);
    }

    #[test]
    fn format_args_request_empty_headers_after_section() {
        let args = minimal_args("https://example.com");
        let out = format_args(&args);
        let after_headers = out.split("  Headers:\n").nth(1).unwrap();
        assert!(
            after_headers.starts_with("  Insecure:"),
            "after Headers: should go to Insecure, got: {after_headers:?}"
        );
    }

    #[test]
    fn format_args_full_output_with_optionals() {
        let mut args = minimal_args("https://example.com/api");
        args.header
            .append(http::header::ACCEPT, "application/json".parse().unwrap());
        args.insecure = true;
        args.request = http::Method::POST;
        args.cacert = Some(PathBuf::from("ca.pem"));
        args.cert = Some(PathBuf::from("cert.pem"));
        args.key = Some(PathBuf::from("key.pem"));
        args.location = true;
        args.requests_per_second = rust_decimal::Decimal::from(10);
        args.duration = Duration::from_secs(120);
        args.max_time = Some(Duration::from_secs(5));
        args.connect_timeout = Some(Duration::from_secs(3));
        args.output = PathBuf::from("out.pb");
        args.protocol = HttpProtocol::Http2;
        args.payload = Some(Payload::Data(Bytes::from_static(b"body")));
        args.upload_file_path = Some(PathBuf::from("upload.txt"));

        let out = format_args(&args);

        format_args_assert!(starts_with, out, "Arguments\n");
        format_args_assert!(contains, out, "  URL: https://example.com/api\n");
        format_args_assert!(contains, out, "    accept: application/json\n");
        format_args_assert!(contains, out, "  Insecure: true\n");
        format_args_assert!(contains, out, "  Method: POST\n");
        format_args_assert!(contains, out, "  CA certificate file: ca.pem\n");
        format_args_assert!(contains, out, "  Certificate file : cert.pem\n");
        format_args_assert!(contains, out, "  Key file: key.pem\n");
        format_args_assert!(contains, out, "  Follow redirects: true\n");
        format_args_assert!(contains, out, "  Throughput: 10 requests/second\n");
        format_args_assert!(contains, out, "  Load test duration: 120 seconds\n");
        format_args_assert!(contains, out, "  Request timeout: 5 seconds\n");
        format_args_assert!(contains, out, "  Connection timeout: 3 seconds\n");
        format_args_assert!(contains, out, "  Output file: out.pb\n");
        format_args_assert!(contains, out, "  Protocol: HTTP/2\n");
        format_args_assert!(contains, out, "  Request body size: 4 bytes\n");
        format_args_assert!(contains, out, "  Upload file: upload.txt\n");
    }
}
