use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Most(usize);

impl Most {
    pub const fn count(self) -> usize {
        self.0
    }

    pub const fn allows(self, count: usize) -> bool {
        count <= self.0
    }
}
