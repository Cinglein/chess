use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SuperbatchCount(usize);

impl SuperbatchCount {
    pub const FIRST: SuperbatchCount = SuperbatchCount(1);

    #[must_use]
    pub const fn count(self) -> usize {
        self.0
    }
}
