use serde::{Deserialize, Serialize};

use super::most::Most;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrivateFnPolicy {
    most: Most,
}

impl PrivateFnPolicy {
    pub const fn most(&self) -> Most {
        self.most
    }
}
