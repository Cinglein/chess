use std::fs;
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;

use crate::workspace::Workspace;

mod test_counts;
mod test_scan;

use test_scan::TestScan;

#[derive(Default)]
pub struct TestBudget {
    files: usize,
    lines: usize,
    tests: usize,
    test_lines: usize,
    violations: Vec<String>,
}

impl TestBudget {
    const MAX_TESTS_PER_FILE: usize = 3;
    const MAX_ASSERTIONS_PER_TEST: usize = 3;
    const MAX_LINES_PER_TEST: usize = 20;
    const MAX_LITERALS_PER_TEST: usize = 4;
    const MAX_INTEGER_LITERAL: u64 = 64;
    const MAX_AVERAGE_TESTS_PER_FILE: usize = 1;
    const MAX_TEST_LINE_PERCENT: usize = 20;

    pub fn check(workspace: &Workspace) -> Result<(), String> {
        workspace
            .rust_files()?
            .iter()
            .map(|file| Self::measure(workspace, file))
            .sum::<Result<Self, String>>()?
            .verdict()
    }

    fn measure(workspace: &Workspace, file: &Path) -> Result<Self, String> {
        let source =
            fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))?;
        let scanned = source
            .contains("test")
            .then(|| syn::parse_file(&source))
            .transpose()
            .map_err(|error| format!("{}: {error}", file.display()))?
            .map(|parsed| TestScan::budget(&workspace.relative(file), &parsed))
            .unwrap_or_default();
        Ok(Self {
            files: 1,
            lines: source.lines().count(),
            ..scanned
        })
    }

    fn verdict(&self) -> Result<(), String> {
        let violations: Vec<String> = self
            .violations
            .iter()
            .cloned()
            .chain(self.average_violation())
            .chain(self.percent_violation())
            .collect();
        if violations.is_empty() {
            println!(
                "test budget: {} tests in {} files, {} of {} lines",
                self.tests, self.files, self.test_lines, self.lines
            );
            Ok(())
        } else {
            Err(format!("test budget exceeded:\n{}", violations.join("\n")))
        }
    }

    fn average_violation(&self) -> Option<String> {
        (self.tests > self.files * Self::MAX_AVERAGE_TESTS_PER_FILE).then(|| {
            format!(
                "{} tests across {} files, at most {} per file on average allowed",
                self.tests,
                self.files,
                Self::MAX_AVERAGE_TESTS_PER_FILE
            )
        })
    }

    fn percent_violation(&self) -> Option<String> {
        (self.test_lines * 100 > self.lines * Self::MAX_TEST_LINE_PERCENT).then(|| {
            format!(
                "{} of {} lines are test code, at most {}% allowed",
                self.test_lines,
                self.lines,
                Self::MAX_TEST_LINE_PERCENT
            )
        })
    }
}

impl Add for TestBudget {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            files: self.files + other.files,
            lines: self.lines + other.lines,
            tests: self.tests + other.tests,
            test_lines: self.test_lines + other.test_lines,
            violations: [self.violations, other.violations].concat(),
        }
    }
}

impl Sum for TestBudget {
    fn sum<I: Iterator<Item = Self>>(budgets: I) -> Self {
        budgets.fold(Self::default(), Self::add)
    }
}

#[cfg(test)]
mod tests {
    use super::TestScan;

    const SOURCE: &str = "
#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let squares = [Square::A1, Square::H8];
        assert_eq!(squares.len(), 2);
        assert!(Bitboard::EMPTY.is_empty());
        assert_ne!(\"a\", \"b\");
        assert_eq!(seed(), 0x2545_F491);
    }
}
";
    const REPORTED: [&str; 3] = [
        "has 4 assertions",
        "has 6 literals",
        "integer literal 0x2545_F491",
    ];

    #[test]
    fn counts_assertions_literals_and_variants_inside_macros() {
        let parsed = syn::parse_file(SOURCE).expect("valid rust");
        let joined = TestScan::budget("example.rs", &parsed)
            .violations
            .join("\n");
        assert!(
            REPORTED.iter().all(|report| joined.contains(report)),
            "{joined}"
        );
    }
}
