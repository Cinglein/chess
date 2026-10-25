use serde::{Deserialize, Serialize};

use super::most::Most;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FnShapePolicy {
    parameters: Most,
    nesting: Most,
}

impl FnShapePolicy {
    pub const fn parameters(&self) -> Most {
        self.parameters
    }

    pub const fn nesting(&self) -> Most {
        self.nesting
    }
}
