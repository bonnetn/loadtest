//! Integration tests: run the loadtest binary and assert on exit code and output.

use assert_cmd::Command;
use predicates::prelude::*;

fn loadtest() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("loadtest")
}

#[test]
fn no_args_shows_help_and_exits_with_error() {
    let mut cmd = loadtest();
    cmd.assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage:"))
        .stderr(predicate::str::contains("<URL>"));
}

#[test]
fn help_succeeds() {
    let mut cmd = loadtest();
    cmd.arg("--help").assert().success();
}

#[test]
fn short_help_succeeds() {
    let mut cmd = loadtest();
    cmd.arg("-h").assert().success();
}

#[test]
fn version_succeeds() {
    let mut cmd = loadtest();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("loadtest"));
}

#[test]
fn short_version_succeeds() {
    let mut cmd = loadtest();
    cmd.arg("-V").assert().success();
}

#[test]
fn url_only_prints_url() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("URL"))
        .stdout(predicate::str::contains("https://example.com"))
        .stdout(predicate::str::contains("Method"));
}

#[test]
fn request_method_override() {
    let mut cmd = loadtest();
    cmd.arg("https://api.example.com")
        .arg("-X")
        .arg("POST")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("POST"));
}

#[test]
fn long_request_method() {
    let mut cmd = loadtest();
    cmd.arg("https://api.example.com")
        .arg("--request")
        .arg("PUT")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("PUT"));
}

#[test]
fn headers_parsed() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("-H")
        .arg("Accept: application/json")
        .arg("--header")
        .arg("X-Custom: value")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("accept"))
        .stdout(predicate::str::contains("application/json"))
        .stdout(predicate::str::contains("x-custom"))
        .stdout(predicate::str::contains("value"));
}

#[test]
fn insecure_flag() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("-k")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Insecure"))
        .stdout(predicate::str::contains("true"));
}

#[test]
fn data_flag() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("-d")
        .arg(r#"{"key":"value"}"#)
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Request body size"))
        .stdout(predicate::str::contains("bytes"));
}

#[test]
fn location_flag() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("-L")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Follow redirects"))
        .stdout(predicate::str::contains("true"));
}

#[test]
fn cacert_and_capath() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("--cacert")
        .arg("/etc/ssl/certs/ca.pem")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("/etc/ssl/certs/ca.pem"));
}

#[test]
fn upload_file() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("-T")
        .arg("/tmp/upload.txt")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("/tmp/upload.txt"));
}

#[test]
fn help_includes_expected_options() {
    let mut cmd = loadtest();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--dry-run"))
        .stdout(predicate::str::contains("--header"))
        .stdout(predicate::str::contains("--insecure"))
        .stdout(predicate::str::contains("--upload-file"))
        .stdout(predicate::str::contains("--request"))
        .stdout(predicate::str::contains("--cacert"))
        .stdout(predicate::str::contains("--data"))
        .stdout(predicate::str::contains("--location"));
}

#[test]
fn help_includes_short_options() {
    let mut cmd = loadtest();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("-H"))
        .stdout(predicate::str::contains("-k"))
        .stdout(predicate::str::contains("-T"))
        .stdout(predicate::str::contains("-X"))
        .stdout(predicate::str::contains("-d"))
        .stdout(predicate::str::contains("-L"));
}

#[test]
fn defaults_when_flags_omitted() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Insecure"))
        .stdout(predicate::str::contains("Follow redirects"));
}

#[test]
fn optional_options_show_none_when_absent() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("URL"))
        .stdout(predicate::str::contains("Method"))
        .stdout(predicate::str::contains("Follow redirects"))
        .stdout(predicate::str::contains("Protocol"));
}

#[test]
fn long_form_data_and_location() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("--data")
        .arg("body")
        .arg("--location")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Request body size"))
        .stdout(predicate::str::contains("Follow redirects"))
        .stdout(predicate::str::contains("true"));
}

#[test]
fn long_form_insecure_and_upload_file() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("--insecure")
        .arg("--upload-file")
        .arg("/path/to/file")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Insecure"))
        .stdout(predicate::str::contains("true"))
        .stdout(predicate::str::contains("/path/to/file"));
}

#[test]
fn all_options_combined() {
    let mut cmd = loadtest();
    cmd.arg("https://api.test/path")
        .arg("-H")
        .arg("A: 1")
        .arg("--header")
        .arg("B: 2")
        .arg("-k")
        .arg("-L")
        .arg("-X")
        .arg("PATCH")
        .arg("-d")
        .arg("payload")
        .arg("--cacert")
        .arg("/ca.pem")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("https://api.test/path"))
        .stdout(predicate::str::contains("a: 1"))
        .stdout(predicate::str::contains("b: 2"))
        .stdout(predicate::str::contains("Insecure"))
        .stdout(predicate::str::contains("Follow redirects"))
        .stdout(predicate::str::contains("PATCH"))
        .stdout(predicate::str::contains("Request body size"))
        .stdout(predicate::str::contains("/ca.pem"));
}

#[test]
fn unknown_flag_fails() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("--unknown-flag")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown").or(predicate::str::contains("Unrecognized")));
}

#[test]
fn version_contains_version_number() {
    let mut cmd = loadtest();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"loadtest \d+\.\d+\.\d+").unwrap());
}

#[test]
fn url_with_special_characters() {
    let mut cmd = loadtest();
    let url = "https://example.com/path?q=1&x=2";
    cmd.arg(url)
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains(url));
}

#[test]
fn empty_headers_list() {
    let mut cmd = loadtest();
    cmd.arg("https://example.com")
        .arg("--http1.1")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Headers"));
}

#[test]
fn request_method_default_in_help() {
    let mut cmd = loadtest();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--request"))
        .stdout(predicate::str::contains("METHOD"));
}
