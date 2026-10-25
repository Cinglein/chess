use std::fmt;

use strum::VariantNames;

use super::Task;
use task::Failure;

pub struct Usage;

impl fmt::Display for Usage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "usage: cargo xtask <{}> [--affected], or cargo xtask mutant-costs <directory>",
            Task::VARIANTS.join("|")
        )
    }
}

impl From<Usage> for Failure {
    fn from(usage: Usage) -> Failure {
        Failure::Usage(usage.to_string())
    }
}
