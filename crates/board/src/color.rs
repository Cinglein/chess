use core::ops::Not;

use enum_map::Enum;
use strum::{EnumCount, EnumIter, FromRepr, VariantArray};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Enum, EnumCount, EnumIter, FromRepr, VariantArray,
)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Color {
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
