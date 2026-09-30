use derive_more::{Add, Display, Sum};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Add, Sum, Display)]
pub struct GameCount(usize);

impl GameCount {
    pub const fn incremented(self) -> GameCount {
        GameCount(self.0 + 1)
    }

    pub const fn count(self) -> usize {
        self.0
    }
}
