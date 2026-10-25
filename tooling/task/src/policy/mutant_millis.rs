use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::millis::Millis;
use crate::crate_name::CrateName;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutantMillis {
    default: Millis,
    crates: BTreeMap<CrateName, Millis>,
}

impl MutantMillis {
    #[must_use]
    pub fn for_crate(&self, name: &CrateName) -> Millis {
        self.crates.get(name).copied().unwrap_or(self.default)
    }
}
