use derive_more::{Add, AddAssign, Display, FromStr, Sum};

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
)]
pub struct NodeCount(u64);

impl NodeCount {
    pub const ZERO: NodeCount = NodeCount(0);
    pub const ONE: NodeCount = NodeCount(1);

    #[must_use]
    pub const fn new(nodes: u64) -> NodeCount {
        NodeCount(nodes)
    }

    #[must_use]
    pub const fn count(self) -> u64 {
        self.0
    }
}

#[cfg(any(test, feature = "proptest"))]
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
