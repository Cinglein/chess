mod test_binary;
mod test_timing;

use std::cmp::Reverse;
use std::time::Duration;

use test_binary::TestBinary;
use test_timing::TestTiming;

use crate::task::failure::Failure;
use crate::task::policy::{Interval, Limit, Policy};
use crate::task::report::Report;
use crate::task::selection::Selection;
use crate::task::site::Site;
use crate::task::violation::Violation;
use crate::task::workspace::Workspace;

pub struct TestTime {
    timings: Vec<TestTiming>,
}

impl TestTime {
    pub fn run(
        workspace: &Workspace,
        selection: &Selection,
        policy: &Policy,
    ) -> Result<(), Failure> {
        if selection.is_empty() {
            println!("test time: no crate affected");
            return Ok(());
        }
        let timings = TestBinary::discover(workspace, selection)?
            .iter()
            .map(TestBinary::timings)
            .collect::<Result<Vec<Vec<TestTiming>>, Failure>>()?
            .into_iter()
            .flatten()
            .collect();
        TestTime { timings }.report(policy).verdict()
    }

    fn report(mut self, policy: &Policy) -> Report {
        self.timings.sort_by_key(|timing| Reverse(timing.elapsed()));
        let total: Duration = self.timings.iter().map(TestTiming::elapsed).sum();
        println!(
            "test time: {} tests in {total:.2?}, slowest {}:",
            self.timings.len(),
            policy.limit(Limit::SlowestShown).count()
        );
        self.timings
            .iter()
            .take(policy.limit(Limit::SlowestShown).count())
            .for_each(|timing| println!("  {timing}"));
        let slow = self
            .timings
            .iter()
            .filter(|timing| timing.elapsed() > policy.interval(Interval::PerTest).duration())
            .map(|timing| {
                Violation::new(
                    Site::Workspace,
                    format!(
                        "{timing}, at most {:?} allowed per test",
                        policy.interval(Interval::PerTest).duration()
                    ),
                )
            });
        let whole = (total > policy.interval(Interval::Total).duration()).then(|| {
            Violation::new(
                Site::Workspace,
                format!(
                    "the suite took {total:.2?}, at most {:?} allowed",
                    policy.interval(Interval::Total).duration()
                ),
            )
        });
        let none = self
            .timings
            .is_empty()
            .then(|| Violation::new(Site::Workspace, "no tests were found to time"));
        Report::new(
            "test time budget exceeded",
            slow.chain(whole).chain(none).collect(),
        )
    }
}
