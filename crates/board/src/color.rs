use core::ops::Not;

use enum_map::Enum;
use strum::{Display, EnumCount, EnumIter, EnumString, FromRepr, VariantArray};

use crate::orthogonal::Orthogonal;
use crate::rank::Rank;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Enum,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    FromRepr,
    VariantArray,
)]
#[repr(u8)]
pub enum Color {
    #[strum(serialize = "w")]
    White,
    #[strum(serialize = "b")]
    Black,
}

impl Color {
    #[must_use]
    pub const fn back_rank(self) -> Rank {
        match self {
            Color::White => Rank::One,
            Color::Black => Rank::Eight,
        }
    }

    #[must_use]
    pub const fn pawn_push_direction(self) -> Orthogonal {
        match self {
            Color::White => Orthogonal::North,
            Color::Black => Orthogonal::South,
        }
    }

    #[must_use]
    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        self.opposite()
    }
}
