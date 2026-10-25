mod tool_scope;
mod tool_trigger;

pub use tool_scope::ToolScope;
pub use tool_trigger::ToolTrigger;

use enum_map::Enum;
use serde::Serialize;
use strum::{Display, EnumIter};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Enum, Display, EnumIter, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[repr(u8)]
pub enum Job {
    Fmt,
    Clippy,
    Test,
    Lint,
    Mutants,
    Full,
}

impl Job {
    const SHARED_ENTRIES: &[&str] = &[
        "xtask/src/main.rs", "xtask/src/lib.rs", "xtask/src/task/mod.rs", "xtask/Cargo.toml",
    ];

    pub fn entries(self) -> impl Iterator<Item = &'static str> {
        let own: &[&str] = match self {
            Job::Fmt | Job::Mutants | Job::Full => &[],
            Job::Clippy => &["xtask/src/task/clippy.rs", "xtask/src/task/wasm.rs"],
            Job::Test => &["xtask/src/task/test.rs"],
            Job::Lint => &["xtask/src/task/lint.rs"],
        };
        Self::SHARED_ENTRIES.iter().chain(own).copied()
    }

    #[must_use]
    pub fn tool_trigger(self) -> ToolTrigger {
        match self {
            Job::Fmt | Job::Clippy | Job::Test | Job::Lint => ToolTrigger::AnyToolChange,
            Job::Mutants | Job::Full => ToolTrigger::ReachedEntry,
        }
    }

    #[must_use]
    pub fn tool_scope(self) -> ToolScope {
        match self {
            Job::Clippy | Job::Test => ToolScope::PlusTheTool,
            Job::Full => ToolScope::Everything,
            Job::Fmt | Job::Lint | Job::Mutants => ToolScope::ProductOnly,
        }
    }
}
