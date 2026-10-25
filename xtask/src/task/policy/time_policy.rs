use serde::{Deserialize, Serialize};

use super::millis::Millis;
use super::most::Most;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimePolicy {
    per_test_millis: Millis,
    total_millis: Millis,
    slowest_shown: Most,
}

impl TimePolicy {
    pub const fn per_test(&self) -> Millis {
        self.per_test_millis
    }

    pub const fn total(&self) -> Millis {
        self.total_millis
    }

    pub const fn slowest_shown(&self) -> Most {
        self.slowest_shown
    }
}
