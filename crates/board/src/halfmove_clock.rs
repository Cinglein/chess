use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr)]
pub struct HalfmoveClock(u8);

impl HalfmoveClock {
    pub const ZERO: HalfmoveClock = HalfmoveClock(0);

    #[must_use]
    pub const fn new(plies: u8) -> Self {
        HalfmoveClock(plies)
    }
}
