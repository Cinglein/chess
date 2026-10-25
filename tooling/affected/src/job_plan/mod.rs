mod packing;

use std::collections::BTreeSet;

use packing::Packing;
use serde::Serialize;

use crate::crate_load::CrateLoad;
use task::CrateName;
use task::Policy;
use task::Selection;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct JobPlan {
    crates: Vec<String>,
    shards: Vec<String>,
    shard_count: usize,
}

impl JobPlan {
    pub fn over(crates: &BTreeSet<CrateName>) -> JobPlan {
        JobPlan {
            crates: crates.iter().map(ToString::to_string).collect(),
            shards: Vec::new(),
            shard_count: 0,
        }
    }

    pub fn sharded(self, loads: &[CrateLoad], policy: &Policy) -> JobPlan {
        let shards: Vec<String> = Packing::shards(loads, policy)
            .iter()
            .map(ToString::to_string)
            .collect();
        JobPlan {
            shard_count: shards.len(),
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
    use super::JobPlan;
    use crate::crate_load::CrateLoad;
    use settings::EmbeddedDefaults;
    use std::collections::BTreeSet;
    use task::{Interval, Policy};
    const SOME: usize = 2;
    const ONE: usize = 1;

    #[test]
    fn shards_follow_the_mutant_counts_and_an_empty_plan_has_none() {
        let policy = Policy::defaults();
        let eval = BTreeSet::from(["eval".parse().unwrap()]);
        let each = policy.interval(Interval::PerShard);
        let load = CrateLoad::new("eval".parse().unwrap(), ONE, each, &policy);
        let some = JobPlan::over(&eval).sharded(core::slice::from_ref(&load), &policy);
        let none = JobPlan::over(&BTreeSet::new()).sharded(&[], &policy);
        assert_eq!(
            (some.shards.len(), some.shard_count, none.shards.len()),
            (SOME, SOME, usize::MIN)
        );
        assert!(none.selection().is_empty() && !some.selection().is_empty());
    }
}
