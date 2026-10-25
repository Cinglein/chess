mod test_binary;
mod test_timing;

use std::cmp::Reverse;
use std::time::Duration;

use test_binary::TestBinary;
use test_timing::TestTiming;

use crate::task::failure::Failure;
use crate::task::policy::TimePolicy;
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
        policy: &TimePolicy,
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

    fn report(mut self, policy: &TimePolicy) -> Report {
        self.timings.sort_by_key(|timing| Reverse(timing.elapsed()));
        let total: Duration = self.timings.iter().map(TestTiming::elapsed).sum();
        println!(
            "test time: {} tests in {total:.2?}, slowest {}:",
            self.timings.len(),
            policy.slowest_shown().count()
        );
        self.timings
            .iter()
            .take(policy.slowest_shown().count())
            .for_each(|timing| println!("  {timing}"));
        let slow = self
            .timings
            .iter()
            .filter(|timing| timing.elapsed() > policy.per_test().duration())
            .map(|timing| {
                Violation::new(
                    Site::Workspace,
                    format!(
                        "{timing}, at most {:?} allowed per test",
                        policy.per_test().duration()
                    ),
                )
            });
        let whole = (total > policy.total().duration()).then(|| {
            Violation::new(
                Site::Workspace,
                format!(
                    "the suite took {total:.2?}, at most {:?} allowed",
                    policy.total().duration()
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
