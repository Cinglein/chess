use std::ops::Range;

use derive_more::{Display, FromStr, Into};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, FromStr, Into)]
pub struct SliceCount(usize);

impl SliceCount {
    #[must_use]
    pub const fn new(slices: usize) -> SliceCount {
        SliceCount(slices)
    }

    #[must_use]
    pub const fn indices(self) -> Range<usize> {
        0..self.0
    }
}
