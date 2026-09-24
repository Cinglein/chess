use std::fmt;

use crate::failure::Failure;
use crate::violation::Violation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    headline: &'static str,
    violations: Vec<Violation>,
}

impl Report {
    pub fn new(headline: &'static str, violations: Vec<Violation>) -> Report {
        Report {
            headline,
            violations,
        }
    }

    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn verdict(self) -> Result<(), Failure> {
        if self.is_clean() {
            Ok(())
        } else {
            Err(Failure::Lint(vec![self]))
        }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}:", self.headline)?;
        self.violations
            .iter()
            .try_for_each(|violation| writeln!(formatter, "{violation}"))
    }
}
