mod memory_watch;
mod mutants_exit;
mod scope;

pub use scope::Scope;

use std::env;
use std::fs::{self, File};

use memory_watch::MemoryWatch;
use mutants_exit::MutantsExit;
use sysinfo::{ProcessesToUpdate, System};

use crate::task::available_gib::AvailableGib;
use crate::task::failure::Failure;
use crate::task::policy::{MemoryShare, Policy};
use crate::task::selection::Selection;
use crate::task::site::Site;
use crate::task::workspace::Workspace;

pub struct Mutants;

impl Mutants {
    const LOCK_FILE: &str = "target/mutants.lock";
    const OUTPUT_DIRECTORY: &str = "target/mutants";
    const PROCESS_NAME: &str = "cargo-mutants";
    const SHARD_VARIABLE: &str = "MUTANTS_SHARD";

    pub fn run(
        workspace: &Workspace,
        scope: Scope,
        selection: &Selection,
        policy: &Policy,
    ) -> Result<(), Failure> {
        if scope == Scope::Workspace && selection.is_empty() {
            println!("mutants: no crate affected");
            return Ok(());
        }
        let _lock = Self::exclusive_lock(workspace)?;
        Self::refuse_if_another_run_exists()?;
        let budget = Self::refuse_if_memory_is_short(policy)?;
        scope.warm_build_directory(workspace, selection)?;
        let arguments: Vec<String> = [
            String::from("mutants"),
            String::from("--jobserver-tasks"),
            budget
                .minus(policy.memory(MemoryShare::ForTheTests))
                .divided_by(policy.memory(MemoryShare::PerCompilerTask))
                .to_string(),
            String::from("--output"),
            String::from(Self::OUTPUT_DIRECTORY),
        ]
        .into_iter()
        .chain(scope.arguments(workspace, selection, policy)?)
        .chain(Self::shard())
        .collect();
        let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
        let mut child = workspace.spawn_cargo(&borrowed)?;
        let status = MemoryWatch::over(&child, budget, policy).guard(&mut child)?;
        MutantsExit::accepts(status)
            .then_some(())
            .ok_or_else(|| Failure::Cargo(borrowed.join(" ")))
    }

    fn shard() -> Vec<String> {
        env::var(Self::SHARD_VARIABLE)
            .ok()
            .map(|shard| vec![String::from("--shard"), shard])
            .unwrap_or_default()
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

    fn refuse_if_memory_is_short(policy: &Policy) -> Result<AvailableGib, Failure> {
        let available = AvailableGib::measured();
        if available < policy.memory(MemoryShare::Granted) {
            return Err(Failure::Refused(format!(
                "{available} available, the granted budget of {} must be free before mutating",
                policy.memory(MemoryShare::Granted)
            )));
        }
        Ok(policy.memory(MemoryShare::Granted))
    }
}
