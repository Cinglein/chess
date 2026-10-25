mod bin;

use std::cmp::Reverse;
use std::collections::BTreeMap;

use bin::Bin;

use task::CrateName;
use task::Shard;
use task::{Limit, Policy};

pub struct Packing;

impl Packing {
    pub fn shards(counts: &BTreeMap<CrateName, usize>, policy: &Policy) -> Vec<Shard> {
        let total: usize = counts.values().sum();
        let shard_count = total
            .div_ceil(policy.limit(Limit::MutantsPerShard).count())
            .min(policy.limit(Limit::MostShards).count());
        if shard_count == 0 {
            return Vec::new();
        }
        let target = total.div_ceil(shard_count);
        let slices: Vec<Shard> = counts
            .iter()
            .filter(|(_, count)| **count > target)
            .flat_map(|(name, count)| Self::slices(name.clone(), count.div_ceil(target)))
            .collect();
        let mut whole: Vec<(&CrateName, &usize)> = counts
            .iter()
            .filter(|(_, count)| **count <= target)
            .collect();
        whole.sort_by_key(|(_, count)| Reverse(**count));
        let bins = whole.into_iter().fold(
            vec![Bin::default(); shard_count.saturating_sub(slices.len()).max(1)],
            |mut bins, (name, count)| {
                Bin::lightest(&mut bins).take(name.clone(), *count);
                bins
            },
        );
        slices
            .into_iter()
            .chain(bins.into_iter().filter_map(Bin::shard))
            .collect()
    }

    fn slices(name: CrateName, count: usize) -> impl Iterator<Item = Shard> {
        (0..count).map(move |index| Shard::Slice {
            crate_name: name.clone(),
            index,
            count,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use settings::EmbeddedDefaults;

    use super::Packing;
    use task::Policy;

    const COUNTS: [(&str, usize); 3] = [("a", 60), ("b", 20), ("c", 10)];
    const SHARDS: [&str; 3] = ["a/0/2", "a/1/2", "b+c"];

    #[test]
    fn crates_above_the_target_are_sliced_and_the_rest_are_packed_largest_first() {
        let counts: BTreeMap<_, _> = COUNTS
            .iter()
            .map(|(name, count)| (name.parse().unwrap(), *count))
            .collect();
        let shards: Vec<String> = Packing::shards(&counts, &Policy::defaults())
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(shards, SHARDS);
    }
}
