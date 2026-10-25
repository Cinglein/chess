mod bin;

use std::cmp::Reverse;

use bin::Bin;

use crate::crate_load::CrateLoad;
use task::Shard;
use task::{Interval, Policy};

pub struct Packing;

impl Packing {
    pub fn shards(loads: &[CrateLoad], policy: &Policy) -> Vec<Shard> {
        let target = policy.interval(Interval::PerShard);
        let slices = loads
            .iter()
            .filter(|load| load.cost() > target)
            .flat_map(|load| Self::slices(load, load.cost().div_ceil(target)));
        let mut whole: Vec<&CrateLoad> =
            loads.iter().filter(|load| load.cost() <= target).collect();
        whole.sort_by_key(|load| Reverse((load.tested_packages(), load.cost())));
        let bins = whole.into_iter().fold(Vec::<Bin>::new(), |mut bins, load| {
            match bins.last_mut().filter(|bin| bin.fits(load.cost(), target)) {
                Some(bin) => bin.take(load.name().clone(), load.cost()),
                None => bins.push(Bin::holding(load.name().clone(), load.cost())),
            }
            bins
        });
        slices.chain(bins.into_iter().map(Bin::shard)).collect()
    }

    fn slices(load: &CrateLoad, count: usize) -> impl Iterator<Item = Shard> {
        let name = load.name().clone();
        (0..count).map(move |index| Shard::Slice {
            crate_name: name.clone(),
            index,
            count,
        })
    }
}

#[cfg(test)]
mod tests {
    use settings::EmbeddedDefaults;

    use super::Packing;
    use crate::crate_load::CrateLoad;
    use task::{Interval, Policy};

    const LOADS: [(&str, usize, usize, Interval); 4] = [
        ("a", 2, 2, Interval::PerShard),
        ("b", 1, 3, Interval::Total),
        ("c", 1, 1, Interval::PerShard),
        ("d", 1, 2, Interval::PerTest),
    ];
    const SHARDS: [&str; 4] = ["a/0/2", "a/1/2", "b+d", "c"];

    #[test]
    fn crates_costlier_than_a_shard_are_sliced_and_the_rest_are_packed_by_shared_test_sets() {
        let policy = Policy::defaults();
        let loads = LOADS.map(|(name, mutants, tested, each)| {
            CrateLoad::new(
                name.parse().unwrap(),
                mutants,
                tested,
                policy.interval(each),
            )
        });
        let shards: Vec<String> = Packing::shards(&loads, &policy)
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(shards, SHARDS);
    }
}
