use std::time::{SystemTime, UNIX_EPOCH};

use derive_more::{Display, From, FromStr, Into};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, From, FromStr, Into, Deserialize)]
#[serde(transparent)]
pub struct Seed(u64);

impl Seed {
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
}
