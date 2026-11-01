mod bits;
mod extent;

pub use extent::{Between, Extent, Through};

use core::fmt;
use core::str::FromStr;

use enum_map::Enum;
use strum::{EnumCount, EnumIter, FromRepr, ParseError, VariantArray};

use crate::bitboard::Bitboard;
use crate::file::File;
use crate::rank::Rank;
use crate::slider::Slider;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Enum,
    EnumCount,
    EnumIter,
    FromRepr,
    VariantArray,
)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    const MIRROR_INDEX_MASK: usize = 0b11_1000;

    #[must_use]
    pub const fn new(file: File, rank: Rank) -> Square {
        Self::VARIANTS[rank as usize * File::COUNT + file as usize]
    }

    #[must_use]
    pub const fn file(self) -> File {
        File::VARIANTS[self as usize % File::COUNT]
    }

    #[must_use]
    pub const fn rank(self) -> Rank {
        Rank::VARIANTS[self as usize / File::COUNT]
    }

    #[must_use]
    pub const fn mirrored(self) -> Square {
        Self::VARIANTS[self as usize ^ Self::MIRROR_INDEX_MASK]
    }

    #[must_use]
    pub fn line<X: Extent>(self, other: Square) -> Bitboard {
        X::trace(self, other)
    }

    pub(super) fn sight<S: Slider>(self, other: Square) -> Option<Bitboard> {
        let open = S::attacks(self, Bitboard::EMPTY);
        open.contains(other).then_some(open)
    }
}

const _: () = {
    let mut squares = Square::VARIANTS;
    while let [square, rest @ ..] = squares {
        assert!(*square as usize == square.rank() as usize * File::COUNT + square.file() as usize);
        assert!(Square::new(square.file(), square.rank()) as u8 == *square as u8);
        squares = rest;
    }
};

impl fmt::Display for Square {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.file(), self.rank())
    }
}

impl FromStr for Square {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Square, ParseError> {
        let (file, rank) = text
            .split_at_checked(1)
            .ok_or(ParseError::VariantNotFound)?;
        Ok(Square::new(file.parse()?, rank.parse()?))
    }
}

#[cfg(feature = "proptest")]
impl proptest::arbitrary::Arbitrary for Square {
    type Parameters = ();
    type Strategy = proptest::sample::Select<Square>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        proptest::sample::select(<Square as strum::VariantArray>::VARIANTS)
    }
}
