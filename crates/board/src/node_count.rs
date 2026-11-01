use core::time::Duration;
use newtype::Newtype;

use derive_more::{Add, AddAssign, Display, FromStr, Sum};
#[cfg(feature = "serde")]
use serde::Deserialize;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Add,
    AddAssign,
    Sum,
    Display,
    FromStr,
    Newtype,
)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(transparent))]
pub struct NodeCount(u64);

impl NodeCount {
    pub const ZERO: NodeCount = NodeCount(0);
    pub const ONE: NodeCount = NodeCount(1);
    const MILLIS_PER_SECOND: u64 = 1000;

    #[must_use]
    pub const fn count(self) -> u64 {
        self.0
    }

    #[must_use]
    pub fn per_second(self, elapsed: Duration) -> NodeCount {
        let millis = u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX);
        NodeCount(self.0.saturating_mul(Self::MILLIS_PER_SECOND) / millis.max(1))
    }
}

#[cfg(feature = "proptest")]
impl proptest::arbitrary::Arbitrary for NodeCount {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<NodeCount>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        proptest::arbitrary::any::<u64>()
            .prop_map(NodeCount::new)
            .boxed()
    }
}
