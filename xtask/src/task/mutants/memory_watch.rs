use std::process::{Child, ExitStatus};
use std::thread;
use std::time::{Duration, Instant};

use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System};

use super::available_gib::AvailableGib;
use crate::task::failure::Failure;
use crate::task::site::Site;

pub struct MemoryWatch {
    root: Pid,
    budget: AvailableGib,
    system: System,
    members: Vec<Pid>,
    last_discovery: Instant,
    killed: Vec<String>,
}

impl MemoryWatch {
    const SAMPLE_EVERY: Duration = Duration::from_millis(20);
    const DISCOVER_EVERY: Duration = Duration::from_millis(100);

    pub fn over(child: &Child, budget: AvailableGib) -> MemoryWatch {
        MemoryWatch {
            root: Pid::from_u32(child.id()),
            budget,
            system: System::new(),
            members: vec![Pid::from_u32(child.id())],
            last_discovery: Instant::now(),
            killed: Vec::new(),
        }
    }

    pub fn guard(mut self, child: &mut Child) -> Result<ExitStatus, Failure> {
        let io = |error| Failure::Io {
            site: Site::File(String::from("cargo-mutants")),
            error,
        };
        let status = loop {
            if let Some(status) = child.try_wait().map_err(io)? {
                break status;
            }
            self.sample();
            thread::sleep(Self::SAMPLE_EVERY);
        };
        if self.killed.is_empty() {
            return Ok(status);
        }
        Err(Failure::Refused(format!(
            "the {} budget was enforced by killing {}; bound the tests that grew under those mutants",
            self.budget,
            self.killed.join(", ")
        )))
    }

    fn sample(&mut self) {
        self.system.refresh_memory();
        if self.last_discovery.elapsed() >= Self::DISCOVER_EVERY {
            self.system.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::nothing().with_memory(),
            );
            self.members = self.discovered_tree();
            self.last_discovery = Instant::now();
        } else {
            self.system.refresh_processes_specifics(
                ProcessesToUpdate::Some(&self.members),
                true,
                ProcessRefreshKind::nothing().with_memory(),
            );
        }
        if let Some(name) = self.enforce() {
            self.killed.push(name);
        }
    }

    fn enforce(&self) -> Option<String> {
        let tree: Vec<&Process> = self
            .members
            .iter()
            .filter_map(|pid| self.system.process(*pid))
            .collect();
        let used: u64 = tree.iter().map(|process| process.memory()).sum();
        if used <= self.budget.minus_reserve().bytes()
            && self.system.available_memory() >= AvailableGib::RESERVE.bytes()
        {
            return None;
        }
        let largest = tree.iter().max_by_key(|process| process.memory())?;
        largest.kill();
        Some(format!(
            "{} (pid {}) at {} of {} used",
            largest.name().to_string_lossy(),
            largest.pid(),
            AvailableGib::from_bytes(largest.memory()),
            AvailableGib::from_bytes(used)
        ))
    }

    fn discovered_tree(&self) -> Vec<Pid> {
        let mut members = vec![self.root];
        loop {
            let grown = self.children_outside(&members);
            if grown.is_empty() {
                break;
            }
            members.extend(grown);
        }
        members
    }

    fn children_outside(&self, members: &[Pid]) -> Vec<Pid> {
        self.system
            .processes()
            .values()
            .filter(|process| !members.contains(&process.pid()))
            .filter(|process| {
                process
                    .parent()
                    .is_some_and(|parent| members.contains(&parent))
            })
            .map(Process::pid)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::{AvailableGib, Failure, MemoryWatch};

    #[test]
    fn a_tree_over_its_budget_is_killed_and_the_run_is_refused() {
        let mut child = Command::new("sleep").arg("30").spawn().unwrap();
        let outcome = MemoryWatch::over(&child, AvailableGib::from_bytes(0)).guard(&mut child);
        assert!(matches!(outcome, Err(Failure::Refused(_))));
    }
}
