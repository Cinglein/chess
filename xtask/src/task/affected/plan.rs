use std::collections::BTreeSet;

use serde::Serialize;

use crate::task::crate_name::CrateName;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Plan {
    crates: Vec<String>,
    shards: Vec<usize>,
    shard_count: usize,
}

impl Plan {
    const MUTANTS_PER_SHARD: usize = 64;
    const MOST_SHARDS: usize = 16;

    pub fn new(crates: &BTreeSet<CrateName>, mutants: usize) -> Plan {
        let shard_count = mutants
            .div_ceil(Self::MUTANTS_PER_SHARD)
            .min(Self::MOST_SHARDS);
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

    use super::Plan;

    const SOME: usize = Plan::MUTANTS_PER_SHARD * 2 + 2;
    const MANY: usize = Plan::MUTANTS_PER_SHARD * Plan::MOST_SHARDS * 2;
    const THREE: usize = 3;

    #[test]
    fn shards_grow_with_the_mutants_up_to_sixteen_and_an_empty_selection_has_none() {
        let eval = BTreeSet::from(["eval".parse().unwrap()]);
        let some = Plan::new(&eval, SOME);
        let many = Plan::new(&eval, MANY);
        let none = Plan::new(&BTreeSet::new(), 0);
        assert_eq!(
            (some.shards.len(), many.shard_count, none.shards.len()),
            (THREE, Plan::MOST_SHARDS, 0)
        );
        assert!(none.crates.is_empty() && !some.crates.is_empty());
    }
}
