mod packing;
mod shard_count;

use std::collections::BTreeSet;

pub use packing::Packing;
use serde::Serialize;
use shard_count::ShardCount;

use crate::crate_load::CrateLoad;
use task::CrateName;
use task::Interval;
use task::Policy;
use task::Selection;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct JobPlan {
    crates: Vec<String>,
    shards: Vec<String>,
    shard_count: ShardCount,
}

impl JobPlan {
    pub fn over(crates: &BTreeSet<CrateName>) -> JobPlan {
        JobPlan {
            crates: crates.iter().map(ToString::to_string).collect(),
            shards: Vec::new(),
            shard_count: ShardCount::new(0),
        }
    }

    pub fn sharded(self, loads: &[CrateLoad], policy: &Policy) -> JobPlan {
        let shards: Vec<String> = Packing::shards(loads, policy.interval(Interval::PerShard))
            .iter()
            .map(ToString::to_string)
            .collect();
        JobPlan {
            shard_count: ShardCount::new(shards.len()),
            shards,
            ..self
        }
    }

    pub fn selection(&self) -> Selection {
        Selection::Crates(
            self.crates
                .iter()
                .filter_map(|name| name.parse().ok())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{JobPlan, ShardCount};
    use crate::crate_load::CrateLoad;
    use settings::EmbeddedDefaults;
    use std::collections::BTreeSet;
    use task::{Interval, MutantCount, Policy};
    const SOME: usize = 2;
    const ONE: usize = 1;

    #[test]
    fn shards_follow_the_mutant_counts_and_an_empty_plan_has_none() {
        let policy = Policy::defaults();
        let eval = BTreeSet::from(["eval".parse().unwrap()]);
        let each = policy.interval(Interval::PerShard);
        let load = CrateLoad::new(
            "eval".parse().unwrap(),
            MutantCount::from(ONE),
            each,
            &policy,
        );
        let some = JobPlan::over(&eval).sharded(core::slice::from_ref(&load), &policy);
        let none = JobPlan::over(&BTreeSet::new()).sharded(&[], &policy);
        assert_eq!(
            (some.shards.len(), some.shard_count, none.shards.len()),
            (SOME, ShardCount::new(SOME), usize::MIN)
        );
        assert!(none.selection().is_empty() && !some.selection().is_empty());
    }
}
