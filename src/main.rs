//! Loadtest binary: parses CLI, prints config (or runs load test via the library).

#![warn(
    clippy::pedantic,
    clippy::cargo_common_metadata,
    clippy::negative_feature_names,
    clippy::redundant_feature_names,
    clippy::wildcard_dependencies,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::as_pointer_underscore,
    clippy::as_underscore,
    clippy::assertions_on_result_states,
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
    clippy::field_scoped_visibility_modifiers,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::indexing_slicing,
    clippy::infinite_loop,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::map_with_unused_argument_over_ranges,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::missing_inline_in_public_items,
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
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_test_prefix,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_and_then,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::semicolon_outside_block,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unused_result_ok,
    clippy::unused_trait_names,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm
)]

use std::io::Write as _;

use console::{Term, style};

use loadtest::{format_args, parse, worker_manager, write_run_report};

fn main() -> miette::Result<()> {
    pin_current_thread(0).map_err(|error| miette::miette!("failed to pin main thread: {error}"))?;
    let runtime = compio::runtime::Runtime::builder()
        .build()
        .map_err(|error| miette::miette!("failed to build Compio runtime: {error}"))?;
    runtime.block_on(run_async())
}

async fn run_async() -> loadtest::Result<()> {
    let args = parse().await?;
    let mut term = Term::stdout();
    let formatted = format_args(&args);
    writeln!(term, "{}", style("Arguments").bold())?;
    for line in formatted.lines().skip(1) {
        writeln!(term, "{line}")?;
    }
    if args.dry_run {
        return Ok(());
    }
    let result = run_loadtest(args.clone()).await?;
    write_run_report(args, result).await
}

async fn run_loadtest(args: loadtest::Args) -> loadtest::Result<loadtest::RunResult> {
    let workers = std::thread::available_parallelism()
        .map(std::num::NonZeroUsize::get)
        .unwrap_or(1);
    let mut senders = Vec::with_capacity(workers);
    let mut receivers = Vec::with_capacity(workers);
    for _ in 0..workers {
        let (sender, receiver) = async_channel::unbounded();
        senders.push(sender);
        receivers.push(receiver);
    }
    let mut threads = Vec::with_capacity(workers);
    let main_receiver = receivers.remove(0);
    let main_executor = compio::runtime::spawn(worker_manager::run_executor_on_current_runtime(
        0,
        args.clone(),
        main_receiver,
    ));

    for (id, receiver) in receivers.into_iter().enumerate() {
        let id = id + 1;
        let args = args.clone();
        threads.push(std::thread::spawn(move || {
            pin_current_thread(id)?;
            worker_manager::run_executor(id, &args, receiver)
        }));
    }

    let result = compio::runtime::spawn(worker_manager::driver_task(args, senders))
        .await
        .map_err(std::io::Error::from)??;
    main_executor.await.map_err(std::io::Error::from)??;
    for thread in threads {
        thread
            .join()
            .map_err(|_| loadtest::AppError::WorkerThreadPanic)??;
    }
    Ok(result)
}

fn pin_current_thread(core: usize) -> std::io::Result<()> {
    #[cfg(target_os = "linux")]
    {
        let mut set = unsafe { std::mem::zeroed::<libc::cpu_set_t>() };
        unsafe {
            libc::CPU_ZERO(&mut set);
            libc::CPU_SET(core, &mut set);
            if libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &set) != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
    }
    Ok(())
}
