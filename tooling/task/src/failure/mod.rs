mod failed_runs;
mod killed_processes;

pub use failed_runs::FailedRuns;
pub use killed_processes::{KilledProcess, KilledProcesses};

use std::io;
use std::path::PathBuf;

use thiserror::Error;

use crate::available_gib::AvailableGib;
use crate::report::Report;
use crate::site::Site;

#[derive(Debug, Error)]
pub enum Failure {
    #[error("{0}")]
    Usage(String),
    #[error("unknown flag {flag}; the only flag is {known}")]
    UnknownFlag { flag: String, known: &'static str },
    #[error("{0} must be set")]
    Unset(&'static str),
    #[error("{0} failed")]
    Command(String),
    #[error("{0}")]
    Runs(FailedRuns),
    #[error("gh failed: {0}")]
    GitHub(String),
    #[error("cargo metadata is not the shape expected: {0}")]
    Metadata(#[from] serde_json::Error),
    #[error("the mutant list is not the shape expected: {0}")]
    MutantList(serde_json::Error),
    #[error("refused: {0}")]
    Refused(String),
    #[error("refused: another cargo xtask mutants holds {}; one run at a time", .0.display())]
    LockHeld(PathBuf),
    #[error("refused: a {0} process is already running; one run at a time")]
    AlreadyRunning(&'static str),
    #[error(
        "refused: {available} available, the granted budget of {granted} must be free before mutating"
    )]
    MemoryShort {
        available: AvailableGib,
        granted: AvailableGib,
    },
    #[error(
        "refused: the {budget} budget was enforced by killing {killed}; bound the tests that grew under those mutants"
    )]
    BudgetEnforced {
        budget: AvailableGib,
        killed: KilledProcesses,
    },
    #[error("{site}: {error}")]
    Io { site: Site, error: io::Error },
    #[error("{site}: {error}")]
    Parse { site: Site, error: syn::Error },
    #[error("{}", .0.iter().map(ToString::to_string).collect::<Vec<String>>().join("\n"))]
    Lint(Vec<Report>),
}
