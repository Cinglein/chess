use std::collections::BTreeSet;

use serde::Serialize;

use crate::task::crate_name::CrateName;
use crate::task::policy::MutationPolicy;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Plan {
    crates: Vec<String>,
    shards: Vec<usize>,
    shard_count: usize,
}

impl Plan {
    pub fn new(crates: &BTreeSet<CrateName>, mutants: usize, policy: &MutationPolicy) -> Plan {
        let shard_count = mutants
            .div_ceil(policy.mutants_per_shard().count())
            .min(policy.most_shards().count());
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
    use crate::task::policy::Policy;

    const SOME: usize = 2;
    const THREE: usize = 3;

    #[test]
    fn shards_grow_with_the_mutants_up_to_the_most_and_an_empty_selection_has_none() {
        let policy = Policy::defaults();
        let per_shard = policy.mutants().mutants_per_shard().count();
        let most = policy.mutants().most_shards().count();
        let eval = BTreeSet::from(["eval".parse().unwrap()]);
        let some = Plan::new(&eval, per_shard * SOME + SOME, policy.mutants());
        let many = Plan::new(&eval, per_shard * most * SOME, policy.mutants());
        let none = Plan::new(&BTreeSet::new(), 0, policy.mutants());
        assert_eq!(
            (some.shards.len(), many.shard_count, none.shards.len()),
            (THREE, most, 0)
        );
        assert!(none.crates.is_empty() && !some.crates.is_empty());
    }
}
