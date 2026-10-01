use std::time::{SystemTime, UNIX_EPOCH};

use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, FromStr, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Seed(u64);

impl Seed {
    #[must_use]
    pub const fn new(seed: u64) -> Seed {
        Seed(seed)
    }

    #[must_use]
    pub fn from_clock() -> Seed {
        Seed(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(u64::MIN, |since| {
                    u64::try_from(since.as_nanos()).unwrap_or(u64::MAX)
                }),
        )
    }

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }
}
