mod least;

use least::Least;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamePolicy {
    shortest_fn: Least,
    shortest_lifetime: Least,
    vague: Vec<String>,
}

impl NamePolicy {
    pub fn is_vague(&self, name: &str) -> bool {
        !self.shortest_fn.allows(name.len()) || self.vague.iter().any(|vague| vague == name)
    }

    pub const fn shortest_lifetime(&self) -> Least {
        self.shortest_lifetime
    }
}
