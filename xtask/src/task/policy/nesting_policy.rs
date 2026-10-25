use serde::{Deserialize, Serialize};

use super::most::Most;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NestingPolicy {
    reexports: Most,
}

impl NestingPolicy {
    pub const fn reexports(&self) -> Most {
        self.reexports
    }
}
