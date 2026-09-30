use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoundCount(usize);

impl RoundCount {
    #[must_use]
    pub const fn new(rounds: usize) -> RoundCount {
        RoundCount(rounds)
    }

    #[must_use]
    pub const fn count(self) -> usize {
        self.0
    }
}
