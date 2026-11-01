use derive_more::{Display, FromStr};
use newtype::Newtype;
#[cfg(feature = "serde")]
use serde::Deserialize;

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, FromStr, Newtype,
)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(transparent))]
pub struct PlyCount(u16);

impl PlyCount {
    pub const ZERO: PlyCount = PlyCount(0);

    #[must_use]
    pub const fn plies(self) -> u16 {
        self.0
    }

    #[must_use]
    pub const fn incremented(self) -> PlyCount {
        PlyCount(self.0.saturating_add(1))
    }
}

#[cfg(feature = "proptest")]
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
