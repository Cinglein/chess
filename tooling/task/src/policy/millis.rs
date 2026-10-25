use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Millis(u64);

impl Millis {
    #[must_use]
    pub const fn duration(self) -> Duration {
        Duration::from_millis(self.0)
    }

    #[must_use]
    pub fn times(self, count: usize) -> Millis {
        Millis(self.0 * u64::try_from(count).unwrap_or(u64::MAX))
    }

    #[must_use]
    pub const fn plus(self, other: Millis) -> Millis {
        Millis(self.0 + other.0)
    }

    #[must_use]
    pub fn div_ceil(self, other: Millis) -> usize {
        usize::try_from(self.0.div_ceil(other.0.max(1))).unwrap_or(usize::MAX)
    }
}
