mod packing;

use std::collections::{BTreeMap, BTreeSet};

use packing::Packing;
use serde::Serialize;

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

    pub fn sharded(self, counts: &BTreeMap<CrateName, usize>, policy: &Policy) -> JobPlan {
        let shards: Vec<String> = Packing::shards(counts, policy)
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
    use std::collections::{BTreeMap, BTreeSet};

    use settings::EmbeddedDefaults;

    use super::JobPlan;
    use task::{Limit, Policy};

    const SOME: usize = 2;
    const THREE: usize = 3;

    #[test]
    fn shards_follow_the_mutant_counts_and_an_empty_plan_has_none() {
        let policy = Policy::defaults();
        let per_shard = policy.limit(Limit::MutantsPerShard).count();
        let eval = BTreeSet::from(["eval".parse().unwrap()]);
        let counts = BTreeMap::from([("eval".parse().unwrap(), per_shard * SOME + SOME)]);
        let some = JobPlan::over(&eval).sharded(&counts, &policy);
        let none = JobPlan::over(&BTreeSet::new()).sharded(&BTreeMap::new(), &policy);
        assert_eq!(
            (some.shards.len(), some.shard_count, none.shards.len()),
            (THREE, THREE, usize::MIN)
        );
        assert!(none.selection().is_empty() && !some.selection().is_empty());
    }
}
