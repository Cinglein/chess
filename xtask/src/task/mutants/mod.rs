mod available_gib;

use std::fs::{self, File};
use std::num::NonZeroUsize;
use std::thread;

use available_gib::AvailableGib;
use sysinfo::{ProcessesToUpdate, System};

use crate::task::failure::Failure;
use crate::task::site::Site;
use crate::task::workspace::Workspace;

pub struct Mutants;

impl Mutants {
    const BASE: &str = "origin/main";
    const DIFF_FILE: &str = "target/mutants.diff";
    const LOCK_FILE: &str = "target/mutants.lock";
    const PROCESS_NAME: &str = "cargo-mutants";
    const JOBS: &str = "2";

    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        let _lock = Self::exclusive_lock(workspace)?;
        Self::refuse_if_another_run_exists()?;
        let compiler_tasks = Self::half_the_cores();
        let _available = Self::refuse_if_memory_is_short(compiler_tasks)?;
        let diff = workspace.git_output(&["diff", "--merge-base", Self::BASE])?;
        fs::write(workspace.root().join(Self::DIFF_FILE), diff).map_err(|error| Failure::Io {
            site: Site::File(Self::DIFF_FILE.to_owned()),
            error,
        })?;
        let compiler_tasks = compiler_tasks.to_string();
        workspace.cargo(&[
            "mutants",
            "--in-diff",
            Self::DIFF_FILE,
            "--jobs",
            Self::JOBS,
            "--jobserver-tasks",
            &compiler_tasks,
        ])
    }

    fn exclusive_lock(workspace: &Workspace) -> Result<File, Failure> {
        let io = |error| Failure::Io {
            site: Site::File(Self::LOCK_FILE.to_owned()),
            error,
        };
        let path = workspace.root().join(Self::LOCK_FILE);
        fs::create_dir_all(path.parent().unwrap_or(workspace.root())).map_err(io)?;
        let lock = File::create(path).map_err(io)?;
        lock.try_lock().map_err(|_| {
            Failure::Refused(format!(
                "another cargo xtask mutants holds {}; one run at a time",
                Self::LOCK_FILE
            ))
        })?;
        Ok(lock)
    }

    fn refuse_if_another_run_exists() -> Result<(), Failure> {
        let mut system = System::new();
        system.refresh_processes(ProcessesToUpdate::All, true);
        if system
            .processes_by_name(Self::PROCESS_NAME.as_ref())
            .next()
            .is_some()
        {
            return Err(Failure::Refused(format!(
                "a {} process is already running; one run at a time",
                Self::PROCESS_NAME
            )));
        }
        Ok(())
    }

    fn refuse_if_memory_is_short(compiler_tasks: usize) -> Result<AvailableGib, Failure> {
        let available = AvailableGib::measured();
        let needed = AvailableGib::needed_for(compiler_tasks);
        if available < needed {
            return Err(Failure::Refused(format!(
                "{available} available, at least {needed} needed for {compiler_tasks} compiler tasks"
            )));
        }
        Ok(available)
    }

    fn half_the_cores() -> usize {
        thread::available_parallelism()
            .map_or(NonZeroUsize::MIN, |cores| {
                NonZeroUsize::new(cores.get() / 2).unwrap_or(NonZeroUsize::MIN)
            })
            .get()
    }
}
