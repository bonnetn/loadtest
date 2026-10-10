use std::io::Write as _;

use console::{Term, style};

use crate::{Args, Result, format_args, parse, worker_manager, write_run_report};

pub struct Tester;

impl Tester {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self) -> Result<()> {
        pin_current_thread(0)?;

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
}

async fn run_loadtest(args: Args) -> Result<crate::RunResult> {
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
            .map_err(|_| crate::AppError::WorkerThreadPanic)??;
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
