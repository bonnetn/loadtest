use miette::{IntoDiagnostic, WrapErr};

use super::worker::Worker;

pub struct Tester;

impl Tester {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> miette::Result<()> {
        let runtime_count = std::thread::available_parallelism()
            .map(std::num::NonZeroUsize::get)
            .unwrap_or(1);
        let mut threads = Vec::with_capacity(runtime_count);

        for id in 0..runtime_count {
            let worker = Worker::new(id);
            let thread = std::thread::Builder::new()
                .name(format!("worker-{id}"))
                .spawn(move || -> miette::Result<()> {
                    pin_current_thread(id);
                    let runtime = compio::runtime::Runtime::builder()
                        .build()
                        .into_diagnostic()
                        .wrap_err_with(|| format!("failed to build runtime {id}"))?;
                    runtime.block_on(worker.run())?;
                    Ok(())
                })
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to spawn worker thread {id}"))?;
            threads.push(thread);
        }

        for thread in threads {
            thread
                .join()
                .map_err(|_| miette::miette!("tester runtime thread panicked"))??;
        }

        Ok(())
    }
}

fn pin_current_thread(core: usize) {
    #[cfg(target_os = "linux")]
    {
        let mut set = unsafe { std::mem::zeroed::<libc::cpu_set_t>() };
        unsafe {
            libc::CPU_ZERO(&mut set);
            libc::CPU_SET(core, &mut set);
            assert_eq!(
                libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &set),
                0,
                "failed to pin worker thread {core}: {}",
                std::io::Error::last_os_error()
            );
        }
    }
}
