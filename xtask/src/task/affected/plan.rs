use std::collections::BTreeSet;

use serde::Serialize;

use crate::task::crate_name::CrateName;
use crate::task::policy::{Limit, Policy};

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Plan {
    crates: Vec<String>,
    shards: Vec<usize>,
    shard_count: usize,
}

impl Plan {
    pub fn new(crates: &BTreeSet<CrateName>, mutants: usize, policy: &Policy) -> Plan {
        let shard_count = mutants
            .div_ceil(policy.limit(Limit::MutantsPerShard).count())
            .min(policy.limit(Limit::MostShards).count());
        Plan {
            crates: crates.iter().map(ToString::to_string).collect(),
            shards: (0..shard_count).collect(),
            shard_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use settings::EmbeddedDefaults;

    use super::Plan;
    use crate::task::policy::{Limit, Policy};

    const SOME: usize = 2;
    const THREE: usize = 3;

    #[test]
    fn shards_grow_with_the_mutants_up_to_the_most_and_an_empty_selection_has_none() {
        let policy = Policy::defaults();
        let per_shard = policy.limit(Limit::MutantsPerShard).count();
        let most = policy.limit(Limit::MostShards).count();
        let eval = BTreeSet::from(["eval".parse().unwrap()]);
        let some = Plan::new(&eval, per_shard * SOME + SOME, &policy);
        let many = Plan::new(&eval, per_shard * most * SOME, &policy);
        let none = Plan::new(&BTreeSet::new(), usize::MIN, &policy);
        assert_eq!(
            (some.shards.len(), many.shard_count, none.shards.len()),
            (THREE, most, usize::MIN)
        );
        assert!(none.crates.is_empty() && !some.crates.is_empty());
    }
}
