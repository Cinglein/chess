use crate::incremented::Incremented;
use derive_more::{Display, FromStr};

use crate::state::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr)]
pub struct HalfmoveClock(u8);

impl State for HalfmoveClock {}

impl HalfmoveClock {
    pub const ZERO: HalfmoveClock = HalfmoveClock(0);

    #[must_use]
    pub const fn new(plies: u8) -> Self {
        HalfmoveClock(plies)
    }
}

impl Incremented for HalfmoveClock {
    fn incremented(self) -> HalfmoveClock {
        HalfmoveClock(self.0.saturating_add(1))
    }
}
