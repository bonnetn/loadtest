#![warn(
    clippy::pedantic,
    // Cargo:
    clippy::cargo_common_metadata,
    clippy::negative_feature_names,
    clippy::redundant_feature_names,
    clippy::wildcard_dependencies,
    // Restrictions:
    // clippy::absolute_paths,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    // clippy::arbitrary_source_item_ordering,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::as_pointer_underscore,
    clippy::as_underscore,
    clippy::assertions_on_result_states,
    // clippy::big_endian_bytes,
    clippy::cfg_not_test,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::default_union_representation,
    clippy::deref_by_slicing,
    clippy::disallowed_script_idents,
    clippy::doc_include_without_cfg,
    clippy::doc_paragraphs_missing_punctuation,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_enum_variants_with_brackets,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::exit,
    // clippy::expect_used,
    clippy::field_scoped_visibility_modifiers,
    clippy::filetype_is_file,
    // clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    // clippy::host_endian_bytes,
    // clippy::if_then_some_else_none,
    // clippy::impl_trait_in_params,
    // clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::infinite_loop,
    // clippy::inline_asm_x86_att_syntax,
    // clippy::inline_asm_x86_intel_syntax,
    clippy::integer_division,
    // clippy::integer_division_remainder_used,
    // clippy::iter_over_hash_type,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    // clippy::little_endian_bytes,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::map_with_unused_argument_over_ranges,
    clippy::mem_forget,
    // clippy::min_ident_chars,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    // clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    // clippy::missing_trait_methods,
    clippy::mixed_read_write_in_expression,
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_raw_strings,
    clippy::non_ascii_literal,
    clippy::non_zero_suggestions,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pathbuf_init_then_push,
    clippy::pattern_type_mismatch,
    clippy::pointer_format,
    clippy::precedence_bits,
    clippy::print_stderr,
    // clippy::print_stdout,
    // clippy::pub_use,
    // clippy::pub_with_shorthand,
    clippy::pub_without_shorthand,
    // clippy::question_mark_used,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_test_prefix,
    clippy::redundant_type_annotations,
    // clippy::ref_patterns,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_and_then,
    clippy::same_name_method,
    clippy::self_named_module_files,
    // clippy::semicolon_inside_block,
    clippy::semicolon_outside_block,
    // clippy::separated_literal_suffix,
    // clippy::shadow_reuse,
    // clippy::shadow_same,
    // clippy::shadow_unrelated,
    // clippy::single_call_fn,
    // clippy::single_char_lifetime_names,
    // clippy::std_instead_of_alloc,
    // clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    // clippy::todo,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    // clippy::unimplemented,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    // clippy::unreachable,
    clippy::unseparated_literal_suffix,
    clippy::unused_result_ok,
    clippy::unused_trait_names,
    // clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
)]
use std::f64::consts::LN_10;
use std::io::Write as _;
use std::time::{Duration, UNIX_EPOCH};

use console::{Term, style};
use prost::Message as _;
use rust_decimal::prelude::ToPrimitive as _;

use crate::cli::Payload;
use crate::error::Result;
use crate::proto::{
    CdfPoint, Header, LoadTestConfig, LoadTestRunReport, WorkerStats as ProtoWorkerStats,
};

mod cli;
mod error;
mod proto;
mod stats;
mod work_unit;
mod worker_manager;

fn main() -> miette::Result<()> {
    run()?;
    Ok(())
}

#[tokio::main]
async fn run() -> Result<()> {
    let args = cli::parse().await?;
    let mut term = Term::stdout();
    print_args(&mut term, &args)?;
    if args.dry_run {
        return Ok(());
    }
    let result = worker_manager::spawn_workers(&args).await?;
    let args = args.clone();
    tokio::task::spawn_blocking(move || write_run_report(&args, &result))
        .await
        .map_err(crate::error::AppError::from)??;
    Ok(())
}

fn print_args(term: &mut Term, args: &cli::Args) -> Result<()> {
    writeln!(term, "{}", style("Arguments").bold())?;
    print_args_request(term, args)?;
    print_args_tls(term, args)?;
    print_args_timing(term, args)?;
    print_args_output(term, args)?;
    Ok(())
}

fn print_args_request(term: &mut Term, args: &cli::Args) -> Result<()> {
    writeln!(
        term,
        "  {}: {}",
        style("URL").bold(),
        style(args.url.to_string()).italic()
    )?;
    writeln!(term, "  {}", style("Headers:").bold())?;
    for (name, value) in &args.header {
        writeln!(
            term,
            "    {}: {}",
            style(name.to_string()).italic(),
            style(value.to_str().unwrap_or("<invalid>")).italic()
        )?;
    }
    writeln!(
        term,
        "  {}: {}",
        style("Insecure").bold(),
        style(args.insecure).italic()
    )?;
    writeln!(
        term,
        "  {}: {}",
        style("Method").bold(),
        style(args.request.to_string()).italic()
    )?;
    Ok(())
}

fn print_args_tls(term: &mut Term, args: &cli::Args) -> Result<()> {
    if let Some(ref cacert) = args.cacert {
        writeln!(
            term,
            "  {}: {}",
            style("CA certificate file").bold(),
            style(cacert.display()).italic()
        )?;
    }
    if let Some(ref cert) = args.cert {
        writeln!(
            term,
            "  {}: {}",
            style("Certificate file ").bold(),
            style(cert.display()).italic()
        )?;
    }
    if let Some(ref key) = args.key {
        writeln!(
            term,
            "  {}: {}",
            style("Key file").bold(),
            style(key.display()).italic()
        )?;
    }
    Ok(())
}

fn print_args_timing(term: &mut Term, args: &cli::Args) -> Result<()> {
    writeln!(
        term,
        "  {}: {}",
        style("Follow redirects").bold(),
        style(args.location).italic()
    )?;
    writeln!(
        term,
        "  {}: {} {}",
        style("Throughput").bold(),
        style(args.requests_per_second).italic(),
        style("requests/second").italic()
    )?;
    writeln!(
        term,
        "  {}: {} {}",
        style("Load test duration").bold(),
        style(args.duration.as_secs()).italic(),
        style("seconds").italic()
    )?;
    if let Some(max_time) = args.max_time {
        writeln!(
            term,
            "  {}: {} {}",
            style("Request timeout").bold(),
            style(max_time.as_secs()).italic(),
            style("seconds").italic()
        )?;
    }
    if let Some(connect_timeout) = args.connect_timeout {
        writeln!(
            term,
            "  {}: {} {}",
            style("Connection timeout").bold(),
            style(connect_timeout.as_secs()).italic(),
            style("seconds").italic()
        )?;
    }
    Ok(())
}

fn print_args_output(term: &mut Term, args: &cli::Args) -> Result<()> {
    writeln!(
        term,
        "  {}: {}",
        style("Output file").bold(),
        style(args.output.display()).italic()
    )?;
    writeln!(
        term,
        "  {}: {}",
        style("Protocol").bold(),
        style(args.protocol.as_ref()).italic()
    )?;
    let payload_size = match args.payload {
        Some(Payload::Data(ref data)) => data.len(),
        Some(Payload::File(ref file)) => file.len(),
        None => 0,
    };
    writeln!(
        term,
        "  {}: {} {}",
        style("Request body size").bold(),
        style(payload_size).italic(),
        style("bytes").italic()
    )?;
    if let Some(ref path) = args.upload_file_path {
        writeln!(
            term,
            "  {}: {}",
            style("Upload file").bold(),
            style(path.display()).italic()
        )?;
    }
    Ok(())
}

fn args_to_config(args: &cli::Args) -> LoadTestConfig {
    let headers: Vec<Header> = args
        .header
        .iter()
        .filter_map(|(name, value)| {
            value.to_str().ok().map(|v| Header {
                name: name.as_str().to_owned(),
                value: v.to_owned(),
            })
        })
        .collect();
    LoadTestConfig {
        url: args.url.to_string(),
        method: args.request.as_str().to_owned(),
        requests_per_second: args.requests_per_second.to_u32().unwrap_or(0),
        duration_secs: i64::try_from(args.duration.as_secs())
            .expect("duration seconds fit in i64"),
        headers,
    }
}

const START_PERCENTILE: f64 = 1.0;
const RESOLUTION: u64 = 16;
const STEPS: u64 = 4;

fn write_run_report(args: &cli::Args, result: &worker_manager::RunResult) -> Result<()> {
    let run_timestamp_nanos: i64 = result
        .run_timestamp
        .duration_since(UNIX_EPOCH)
        .expect("run timestamp is before UNIX epoch")
        .as_nanos()
        .try_into()
        .expect("run timestamp overflow (nanos do not fit i64)");

    let all_latencies: Vec<std::time::Duration> = result
        .success_latencies
        .iter()
        .chain(result.non_success_latencies.iter())
        .copied()
        .collect();
    let cdf_all = calculate_cdf(START_PERCENTILE, RESOLUTION, STEPS, all_latencies.clone());
    let cdf_success = calculate_cdf(
        START_PERCENTILE,
        RESOLUTION,
        STEPS,
        result.success_latencies.clone(),
    );
    let cdf_non_success = calculate_cdf(
        START_PERCENTILE,
        RESOLUTION,
        STEPS,
        result.non_success_latencies.clone(),
    );

    let to_cdf_proto = |cdf: Vec<(f64, std::time::Duration)>| {
        cdf.into_iter()
            .map(|(p, d)| CdfPoint {
                percentile: p,
                latency_nanos: i64::try_from(d.as_nanos()).unwrap_or(i64::MAX),
            })
            .collect::<Vec<_>>()
    };

    let cdf_proto = to_cdf_proto(cdf_all.clone());
    let cdf_success_proto = to_cdf_proto(cdf_success.clone());
    let cdf_non_success_proto = to_cdf_proto(cdf_non_success.clone());

    print_cdfs(&cdf_all, &cdf_success, &cdf_non_success);

    let worker_stats_proto: Vec<ProtoWorkerStats> = result
        .worker_stats
        .iter()
        .map(|w| ProtoWorkerStats {
            timestamp_unix_nanos: w
                .timestamp
                .duration_since(UNIX_EPOCH)
                .map(|d| i64::try_from(d.as_nanos()).expect("timestamp nanos fit in i64"))
                .unwrap_or(0),
            worker_id: u32::try_from(w.id).expect("worker id fits in u32"),
            elapsed_nanos: w.elapsed.as_nanos().try_into().unwrap_or(u64::MAX),
            request_sent: u64::try_from(w.request_sent).expect("request_sent fits in u64"),
            in_flight: u64::try_from(w.in_flight).expect("in_flight fits in u64"),
            informational_response: w.informational_response,
            successful_response: w.successful_response,
            redirection_message: w.redirection_message,
            client_error_response: w.client_error_response,
            server_error_response: w.server_error_response,
            other_error_response: w.other_error_response,
            timeouts: w.timeouts,
        })
        .collect();

    let report = LoadTestRunReport {
        run_timestamp_unix_nanos: run_timestamp_nanos,
        config: Some(args_to_config(args)),
        worker_stats: worker_stats_proto,
        cdf: cdf_proto,
        cdf_success: cdf_success_proto,
        cdf_non_success: cdf_non_success_proto,
    };

    let mut buf = Vec::new();
    report.encode(&mut buf)?;
    let path = &args.output;
    let mut f = std::fs::File::create(path)?;
    f.write_all(&buf)?;
    println!("Wrote run report to {}", path.display());
    Ok(())
}

fn calculate_cdf(
    start: f64,
    resolution: u64,
    steps: u64,
    mut durations: Vec<Duration>,
) -> Vec<(f64, Duration)> {
    durations.sort();

    let resolution_f64 = f64::from(u32::try_from(resolution).expect("resolution fits in u32 for CDF"));
    let k = -LN_10 / resolution_f64;
    let ratio = k.exp();
    let mut r = start;

    let n = steps
        .checked_mul(resolution)
        .expect("CDF step count overflow");
    let n_usize = usize::try_from(n).expect("CDF step count fits in usize");
    let mut result = Vec::with_capacity(n_usize);

    let len = durations.len();
    for _ in 0..=n {
        let p = 1.0_f64 - r;
        let p_scaled = (p * 1e9).round().clamp(0.0, 1e9);
        let p_scaled = format!("{p_scaled:.0}").parse::<u64>().expect("p_scaled in 0..=1e9 parses as u64");
        let idx = u64::try_from(len).expect("len fits in u64")
            .checked_mul(p_scaled)
            .and_then(|v| v.checked_div(1_000_000_000))
            .expect("CDF index overflow");
        let idx = usize::try_from(idx).expect("CDF index fits in usize");
        if let Some(value) = durations.get(idx) {
            result.push((p, *value));
        }
        r *= ratio;
    }

    result
}

fn print_cdfs(
    cdf_all: &[(f64, std::time::Duration)],
    cdf_success: &[(f64, std::time::Duration)],
    cdf_non_success: &[(f64, std::time::Duration)],
) {
    let fmt = |cdf: &[(f64, std::time::Duration)]| {
        cdf.iter()
            .map(|&(p, d)| format!("[{}, {}]", p, d.as_nanos()))
            .collect::<Vec<String>>()
            .join(", ")
    };
    println!("cdf (all): [{}]", fmt(cdf_all));
    println!("cdf (success): [{}]", fmt(cdf_success));
    println!("cdf (non-success): [{}]", fmt(cdf_non_success));
}
