mod driver;
mod tool_scope;

pub use driver::Driver;
pub use tool_scope::ToolScope;

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
    #[must_use]
    pub fn driver(self) -> Driver {
        match self {
            Job::Fmt | Job::Clippy | Job::Test | Job::Lint => Driver::Cargo,
            Job::Mutants | Job::Full => Driver::CargoMutants,
        }
    }

    #[must_use]
    pub fn tool_scope(self) -> ToolScope {
        match self {
            Job::Full => ToolScope::Everything,
            Job::Fmt | Job::Clippy | Job::Test | Job::Lint | Job::Mutants => ToolScope::ProductOnly,
        }
    }
}
