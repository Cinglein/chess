use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Display, FromStr)]
pub struct ShuffleSeed(u64);

impl ShuffleSeed {
    #[must_use]
    pub const fn new(seed: u64) -> ShuffleSeed {
        ShuffleSeed(seed)
    }

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }
}
