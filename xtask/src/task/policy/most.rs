use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Most(usize);

impl Most {
    pub const fn count(self) -> usize {
        self.0
    }
}
