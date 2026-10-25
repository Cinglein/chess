use std::fmt;
use std::time::Duration;

pub(super) struct TestTiming {
    binary: String,
    test: String,
    elapsed: Duration,
}

impl TestTiming {
    pub(super) const fn new(binary: String, test: String, elapsed: Duration) -> TestTiming {
        TestTiming {
            binary,
            test,
            elapsed,
        }
    }

    pub(super) const fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

impl fmt::Display for TestTiming {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {} took {:.1?}",
            self.binary, self.test, self.elapsed
        )
    }
}
