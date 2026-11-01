use derive_more::{Display, FromStr};
#[cfg(feature = "serde")]
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Display, FromStr)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(transparent))]
pub struct Elo(u16);

impl Elo {
    #[must_use]
    pub const fn new(rating: u16) -> Elo {
        Elo(rating)
    }
}
