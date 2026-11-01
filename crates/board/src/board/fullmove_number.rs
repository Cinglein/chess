use core::num::NonZeroU16;
use newtype::Newtype;

use derive_more::{Display, FromStr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr, Newtype)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct FullmoveNumber(NonZeroU16);

impl State for FullmoveNumber {}

impl FullmoveNumber {
    pub const FIRST: FullmoveNumber = FullmoveNumber(NonZeroU16::MIN);

    #[must_use]
    pub const fn following(self) -> FullmoveNumber {
        FullmoveNumber(self.0.saturating_add(1))
    }
}
