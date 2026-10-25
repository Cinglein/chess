use std::fmt;

use crate::failure::Failure;
use crate::violation::Violation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    headline: &'static str,
    violations: Vec<Violation>,
}

impl Report {
    #[must_use]
    pub fn new(headline: &'static str, violations: Vec<Violation>) -> Report {
        Report {
            headline,
            violations,
        }
    }

    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }

    #[must_use]
    pub fn lists_exactly(&self, expected: &[&str]) -> bool {
        self.violations.len() == expected.len()
            && self
                .violations
                .iter()
                .zip(expected)
                .all(|(violation, text)| violation.to_string().contains(text))
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
