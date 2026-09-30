use derive_more::{Add, Sum};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Add, Sum)]
pub(super) struct PairCount(usize);

impl PairCount {
    pub(super) const fn incremented(self) -> PairCount {
        PairCount(self.0 + 1)
    }

    pub(super) fn as_float(self) -> f64 {
        f64::from(u32::try_from(self.0).unwrap_or(u32::MAX))
    }
}
