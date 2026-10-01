use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, FromStr)]
pub struct Depth(u8);

impl Depth {
    pub const ZERO: Depth = Depth(0);

    #[must_use]
    pub const fn new(plies: u8) -> Depth {
        Depth(plies)
    }

    #[must_use]
    pub const fn plies(self) -> u8 {
        self.0
    }

    #[must_use]
    pub const fn incremented(self) -> Depth {
        Depth(self.0.saturating_add(1))
    }

    #[must_use]
    pub const fn decremented(self) -> Option<Depth> {
        match self.0.checked_sub(1) {
            Some(plies) => Some(Depth(plies)),
            None => None,
        }
    }
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for Depth {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Depth>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        proptest::arbitrary::any::<u8>()
            .prop_map(Depth::new)
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::Depth;

    #[test]
    fn a_depth_reads_back_its_plies_and_a_step_down_then_up_returns_to_itself() {
        proptest!(|(plies: u8)| {
            let depth = Depth::new(plies);
            prop_assert_eq!(
                (depth.plies(), depth.to_string().parse(), depth.decremented().map(Depth::incremented)),
                (plies, Ok(depth), (plies > 0).then_some(depth))
            );
        });
    }
}
