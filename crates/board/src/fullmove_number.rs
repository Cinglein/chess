use core::num::NonZeroU16;

use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr)]
pub struct FullmoveNumber(NonZeroU16);

impl FullmoveNumber {
    pub const FIRST: FullmoveNumber = FullmoveNumber(NonZeroU16::MIN);

    #[must_use]
    pub const fn new(number: NonZeroU16) -> Self {
        FullmoveNumber(number)
    }
}
