use serde::{Deserialize, Serialize};

use super::millis::Millis;
use super::most::Most;
use crate::task::available_gib::AvailableGib;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationPolicy {
    granted_gib: AvailableGib,
    reserve_gib: AvailableGib,
    gib_per_compiler_task: AvailableGib,
    gib_for_the_tests: AvailableGib,
    parallel_jobs: Most,
    sample_every_millis: Millis,
    discover_every_millis: Millis,
    mutants_per_shard: Most,
    most_shards: Most,
}

impl MutationPolicy {
    pub const fn granted(&self) -> AvailableGib {
        self.granted_gib
    }

    pub const fn reserve(&self) -> AvailableGib {
        self.reserve_gib
    }

    pub const fn per_compiler_task(&self) -> AvailableGib {
        self.gib_per_compiler_task
    }

    pub const fn for_the_tests(&self) -> AvailableGib {
        self.gib_for_the_tests
    }

    pub const fn parallel_jobs(&self) -> Most {
        self.parallel_jobs
    }

    pub const fn sample_every(&self) -> Millis {
        self.sample_every_millis
    }

    pub const fn discover_every(&self) -> Millis {
        self.discover_every_millis
    }

    pub const fn mutants_per_shard(&self) -> Most {
        self.mutants_per_shard
    }

    pub const fn most_shards(&self) -> Most {
        self.most_shards
    }
}
