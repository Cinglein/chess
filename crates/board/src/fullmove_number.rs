use core::num::NonZeroU16;

use derive_more::{Display, FromStr};

use crate::state::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr)]
pub struct FullmoveNumber(NonZeroU16);

impl State for FullmoveNumber {}

impl FullmoveNumber {
    pub const FIRST: FullmoveNumber = FullmoveNumber(NonZeroU16::MIN);

    #[must_use]
    pub const fn new(number: NonZeroU16) -> Self {
        FullmoveNumber(number)
    }

    #[must_use]
    pub const fn incremented(self) -> Self {
        FullmoveNumber(self.0.saturating_add(1))
    }
}
