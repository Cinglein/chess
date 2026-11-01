use derive_more::{Display, FromStr};
use newtype::Newtype;

use crate::state::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr, Newtype)]
pub struct HalfmoveClock(u8);

impl State for HalfmoveClock {}

impl HalfmoveClock {
    pub const ZERO: HalfmoveClock = HalfmoveClock(0);

    #[must_use]
    pub const fn incremented(self) -> HalfmoveClock {
        HalfmoveClock(self.0.saturating_add(1))
    }
}
