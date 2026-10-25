mod memory_watch;
mod mutants_exit;
mod scope;

pub use scope::Scope;

use std::fs::{self, File};

use memory_watch::MemoryWatch;
use mutants_exit::MutantsExit;
use sysinfo::{ProcessesToUpdate, System};

use task::AvailableGib;
use task::Failure;
use task::Selection;
use task::Site;
use task::Workspace;
use task::{CargoArgument, CommandLine, MutantsArgument, Program};
use task::{MemoryShare, Policy};

pub struct Mutants;

impl Mutants {
    const LOCK_FILE: &str = "target/mutants.lock";
    const PROCESS_NAME: &str = "cargo-mutants";

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
        let line = workspace
            .line(Program::Cargo)
            .argument(CargoArgument::Mutants)
            .argument(MutantsArgument::JobserverTasks)
            .argument(
                budget
                    .minus(policy.memory(MemoryShare::ForTheTests))
                    .divided_by(policy.memory(MemoryShare::PerCompilerTask)),
            );
        let lines = scope.applied(line, workspace, selection)?;
        let failed: Vec<String> = lines
            .iter()
            .map(|line| Self::accepted(line, budget, policy))
            .collect::<Result<Vec<bool>, Failure>>()?
            .into_iter()
            .zip(&lines)
            .filter(|(accepted, _)| !accepted)
            .map(|(_, line)| line.to_string())
            .collect();
        failed
            .is_empty()
            .then_some(())
            .ok_or_else(|| Failure::Command(failed.join("\n")))
    }

    fn accepted(
        line: &CommandLine,
        budget: AvailableGib,
        policy: &Policy,
    ) -> Result<bool, Failure> {
        let mut child = line.spawn()?;
        let status = MemoryWatch::over(&child, budget, policy).guard(&mut child)?;
        Ok(MutantsExit::accepts(status))
    }

    fn exclusive_lock(workspace: &Workspace) -> Result<File, Failure> {
        let io = |error| Failure::Io {
            site: Site::File(Self::LOCK_FILE.into()),
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
