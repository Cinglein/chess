use newtype::Newtype;
use std::time::Duration;

use derive_more::{Add, Display};
use serde::Deserialize;

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Add, Display, Deserialize, Newtype,
)]
#[serde(transparent)]
pub struct PositionCount(usize);

impl PositionCount {
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
