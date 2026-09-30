use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Seed(u64);

impl Seed {
    #[must_use]
    pub const fn new(seed: u64) -> Seed {
        Seed(seed)
    }

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }
}
