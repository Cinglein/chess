use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Display, FromStr)]
pub struct Elo(u16);

impl Elo {
    #[must_use]
    pub const fn new(rating: u16) -> Elo {
        Elo(rating)
    }
}
