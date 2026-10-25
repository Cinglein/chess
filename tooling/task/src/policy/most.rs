use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Most(usize);

impl Most {
    pub const fn count(self) -> usize {
        self.0
    }

    pub fn counted<Count: From<usize>>(self) -> Count {
        Count::from(self.0)
    }

    pub fn allows(self, count: impl Into<usize>) -> bool {
        count.into() <= self.0
    }
}
