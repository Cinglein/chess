mod test_binary;
mod test_timing;

use std::cmp::Reverse;
use std::time::Duration;

use test_binary::TestBinary;
use test_timing::TestTiming;

use crate::task::failure::Failure;
use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::violation::Violation;
use crate::task::workspace::Workspace;

pub struct TestTime {
    timings: Vec<TestTiming>,
}

impl TestTime {
    const MAX_PER_TEST: Duration = Duration::from_secs(1);
    const MAX_TOTAL: Duration = Duration::from_secs(4);
    const SLOWEST_SHOWN: usize = 5;

    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        let timings = TestBinary::discover(workspace)?
            .iter()
            .map(TestBinary::timings)
            .collect::<Result<Vec<Vec<TestTiming>>, Failure>>()?
            .into_iter()
            .flatten()
            .collect();
        TestTime { timings }.report().verdict()
    }

    fn report(mut self) -> Report {
        self.timings.sort_by_key(|timing| Reverse(timing.elapsed()));
        let total: Duration = self.timings.iter().map(TestTiming::elapsed).sum();
        println!(
            "test time: {} tests in {total:.2?}, slowest {}:",
            self.timings.len(),
            Self::SLOWEST_SHOWN
        );
        self.timings
            .iter()
            .take(Self::SLOWEST_SHOWN)
            .for_each(|timing| println!("  {timing}"));
        let slow = self
            .timings
            .iter()
            .filter(|timing| timing.elapsed() > Self::MAX_PER_TEST)
            .map(|timing| {
                Violation::new(
                    Site::Workspace,
                    format!(
                        "{timing}, at most {:?} allowed per test",
                        Self::MAX_PER_TEST
                    ),
                )
            });
        let whole = (total > Self::MAX_TOTAL).then(|| {
            Violation::new(
                Site::Workspace,
                format!(
                    "the suite took {total:.2?}, at most {:?} allowed",
                    Self::MAX_TOTAL
                ),
            )
        });
        Report::new("test time budget exceeded", slow.chain(whole).collect())
    }
}
