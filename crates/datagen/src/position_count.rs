use std::time::Duration;

use derive_more::{Add, Display};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    Display,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct PositionCount(usize);

impl PositionCount {
    #[must_use]
    pub const fn new(positions: usize) -> PositionCount {
        PositionCount(positions)
    }

    #[must_use]
    pub const fn count(self) -> usize {
        self.0
    }

    #[must_use]
    pub fn per_second(self, elapsed: Duration) -> PositionCount {
        PositionCount(
            self.0
                / usize::try_from(elapsed.as_secs()).map_or(usize::MAX, |seconds| seconds.max(1)),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::PositionCount;

    const TEN: PositionCount = PositionCount::new(10);
    const FIVE: PositionCount = PositionCount::new(5);
    const TWO_SECONDS: Duration = Duration::from_secs(2);

    #[test]
    fn a_rate_divides_by_whole_seconds_and_a_run_under_a_second_counts_as_one() {
        assert_eq!(
            (TEN.per_second(TWO_SECONDS), TEN.per_second(Duration::ZERO)),
            (FIVE, TEN)
        );
    }
}
