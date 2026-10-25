use std::iter::Sum;
use std::ops::Add;

use crate::task::policy::{Policy, Share};
use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::support_modules::SupportModules;
use crate::task::violation::Violation;

mod measurement;
mod test_scan;

use measurement::Measurement;
use test_scan::TestScan;

#[derive(Default)]
pub struct TestBudget {
    files: usize,
    lines: usize,
    tests: usize,
    test_lines: usize,
    violations: Vec<Violation>,
}

impl TestBudget {
    pub fn report(files: &[SourceFile], policy: &Policy) -> Report {
        let support: Vec<String> = files.iter().flat_map(SupportModules::paths).collect();
        files
            .iter()
            .map(|file| Self::measure(file, &support, policy))
            .sum::<Self>()
            .summarise(policy)
    }

    fn measure(file: &SourceFile, support: &[String], policy: &Policy) -> Self {
        let lines = file.text().lines().count();
        let scanned = TestScan::budget(file.path(), file.syntax(), policy);
        Self {
            files: 1,
            lines,
            test_lines: if support.contains(&file.path().to_owned()) {
                lines
            } else {
                scanned.test_lines
            },
            ..scanned
        }
    }

    fn summarise(self, policy: &Policy) -> Report {
        println!(
            "test budget: {} tests in {} files, {} of {} lines",
            self.tests, self.files, self.test_lines, self.lines
        );
        let global = [
            Measurement::new(
                "tests across the workspace",
                self.tests,
                policy
                    .share(Share::TestsPerHundredFiles)
                    .applied_to(self.files),
            ),
            Measurement::new(
                "test lines",
                self.test_lines,
                policy.share(Share::TestLinePercent).applied_to(self.lines),
            ),
        ]
        .into_iter()
        .filter(Measurement::exceeded)
        .map(|measurement| Violation::new(Site::Workspace, measurement.to_string()));
        Report::new(
            "test budget exceeded",
            self.violations.into_iter().chain(global).collect(),
        )
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
    use settings::EmbeddedDefaults;

    use super::TestScan;
    use crate::task::policy::Policy;

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

    #[test]
    fn property() {
        proptest!(|(count in 0..100usize, flag: bool)| {
            prop_assert!(count < 100 || flag);
        });
    }

    fn helper() {
        assert!(true);
    }
}
";
    const REPORTED: [&str; 5] = [
        "has 4 assertions",
        "has 6 literals",
        "integer literal 0x2545_F491",
        "integer literal 100usize",
        "assertion outside a test",
    ];

    #[test]
    fn counts_assertions_literals_and_variants_inside_macros() {
        let parsed = syn::parse_file(SOURCE).expect("valid rust");
        let joined: Vec<String> = TestScan::budget("example.rs", &parsed, &Policy::defaults())
            .violations
            .iter()
            .map(ToString::to_string)
            .collect();
        let joined = joined.join("\n");
        assert!(
            REPORTED.iter().all(|report| joined.contains(report)),
            "{joined}"
        );
    }
}
