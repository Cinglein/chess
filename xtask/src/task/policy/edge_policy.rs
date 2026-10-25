use serde::{Deserialize, Serialize};

use super::most::Most;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgePolicy {
    edges_per_vertex: Most,
}

impl EdgePolicy {
    pub const fn edges_per_vertex(&self) -> Most {
        self.edges_per_vertex
    }
}
