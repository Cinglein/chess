use derive_more::{Display, FromStr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, FromStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct PlyCount(u16);

impl PlyCount {
    pub const ZERO: PlyCount = PlyCount(0);

    #[must_use]
    pub const fn new(plies: u16) -> PlyCount {
        PlyCount(plies)
    }

    #[must_use]
    pub const fn plies(self) -> u16 {
        self.0
    }

    #[must_use]
    pub const fn incremented(self) -> PlyCount {
        PlyCount(self.0.saturating_add(1))
    }
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for PlyCount {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<PlyCount>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        proptest::arbitrary::any::<u16>()
            .prop_map(PlyCount::new)
            .boxed()
    }
}
