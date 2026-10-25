use std::collections::BTreeSet;

use serde::Serialize;

use task::CrateName;
use task::Selection;
use task::{Limit, Policy};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct JobPlan {
    crates: Vec<String>,
    shards: Vec<usize>,
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

    pub fn sharded(self, mutants: usize, policy: &Policy) -> JobPlan {
        let shard_count = mutants
            .div_ceil(policy.limit(Limit::MutantsPerShard).count())
            .min(policy.limit(Limit::MostShards).count());
        JobPlan {
            shards: (0..shard_count).collect(),
            shard_count,
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
    use std::collections::BTreeSet;

    use settings::EmbeddedDefaults;

    use super::JobPlan;
    use task::{Limit, Policy};

    const SOME: usize = 2;
    const THREE: usize = 3;

    #[test]
    fn shards_grow_with_the_mutants_up_to_the_most_and_an_empty_plan_has_none() {
        let policy = Policy::defaults();
        let per_shard = policy.limit(Limit::MutantsPerShard).count();
        let most = policy.limit(Limit::MostShards).count();
        let eval = BTreeSet::from(["eval".parse().unwrap()]);
        let some = JobPlan::over(&eval).sharded(per_shard * SOME + SOME, &policy);
        let many = JobPlan::over(&eval).sharded(per_shard * most * SOME, &policy);
        let none = JobPlan::over(&BTreeSet::new()).sharded(usize::MIN, &policy);
        assert_eq!(
            (some.shards.len(), many.shard_count, none.shards.len()),
            (THREE, most, usize::MIN)
        );
        assert!(none.selection().is_empty() && !some.selection().is_empty());
    }
}
