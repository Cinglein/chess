use std::collections::BTreeSet;

use task::CrateName;
use task::Shard;

#[derive(Clone, Debug, Default)]
pub struct Bin {
    load: usize,
    crates: BTreeSet<CrateName>,
}

impl Bin {
    pub fn lightest(bins: &mut [Bin]) -> &mut Bin {
        bins.iter_mut()
            .min_by_key(|bin| bin.load)
            .expect("a packing has at least one bin")
    }

    pub fn take(&mut self, name: CrateName, count: usize) {
        self.load += count;
        self.crates.insert(name);
    }

    pub fn shard(self) -> Option<Shard> {
        (!self.crates.is_empty()).then_some(Shard::Whole(self.crates))
    }
}
