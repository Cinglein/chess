use std::iter::Sum;
use std::ops::Add;
use std::path::PathBuf;

use task::Report;
use task::Site;
use task::SourceFile;
use task::SupportModules;
use task::Violation;
use task::{Policy, Share};

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
        let support: Vec<PathBuf> = files.iter().flat_map(SupportModules::paths).collect();
        files
            .iter()
            .map(|file| Self::measure(file, &support, policy))
            .sum::<Self>()
            .summarise(policy)
    }

    fn measure(file: &SourceFile, support: &[PathBuf], policy: &Policy) -> Self {
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

    use super::TestBudget;
    use task::Policy;
    use task::SourceFile;

    const LIB: &str = "
#[cfg(test)]
mod fixtures;
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
    #[test]
    fn edge() { assert!(true); assert!(true); assert_eq!(64, 64); }
    fn helper() -> Vec<u8> { assert!(true); vec![1] }
}
";
    const FIXTURES: &str = "pub struct Fixture;\npub struct Other;";
    const LONG_HEAD: &str = "
#[cfg(test)]
mod tests {
    #[test] fn one() {}
    #[test] fn two() {}
    #[test] fn three() {}
    #[test]
    fn long() {";
    const LONG_TAIL: &str = "    }\n}\n";
    const PADDING: usize = 22;
    const PATHS: [&str; 3] = [
        "crates/a/src/lib.rs",
        "crates/a/src/fixtures.rs",
        "crates/a/src/long.rs",
    ];
    const REPORTED: [&str; 10] = [
        "lib.rs:7: fn example has 4 assertions, at most 3 allowed",
        "lib.rs:7: fn example has 6 literals, at most 4 allowed",
        "lib.rs:12: integer literal 0x2545_F491 in test code, at most 64 allowed",
        "lib.rs:16: integer literal 100usize in test code",
        "lib.rs:17: integer literal 100 in test code",
        "lib.rs:22: assertion outside a test",
        "long.rs:8: fn long has 21 body lines, at most 20 allowed",
        "long.rs: 4 tests, at most 3 allowed",
        "workspace: has 7 tests across the workspace, at most 1 allowed",
        "workspace: has 54 test lines, at most 10 allowed",
    ];

    #[test]
    fn measures_tests_files_and_lines_and_flags_every_budget_overrun() {
        let long = format!("{LONG_HEAD}{}{LONG_TAIL}", "\n".repeat(PADDING));
        let files: Vec<SourceFile> = PATHS
            .into_iter()
            .zip([LIB.to_owned(), FIXTURES.to_owned(), long])
            .map(|(path, text)| SourceFile::parse(path.to_owned(), text).expect("valid rust"))
            .collect();
        let report = TestBudget::report(&files, &Policy::defaults());
        assert!(report.lists_exactly(&REPORTED), "{report}");
    }
}
