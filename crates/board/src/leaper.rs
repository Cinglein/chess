mod king;
mod knight;
mod pawn;
mod table;

use enum_map::Enum;
use strum::{EnumCount, EnumIter, FromRepr, VariantArray};

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::square::Square;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Enum, EnumCount, EnumIter, FromRepr, VariantArray,
)]
#[repr(u8)]
pub enum Leaper {
    Knight,
    King,
    WhitePawn,
    BlackPawn,
}

impl Leaper {
    #[must_use]
    pub const fn pawn(color: Color) -> Leaper {
        match color {
            Color::White => Leaper::WhitePawn,
            Color::Black => Leaper::BlackPawn,
        }
    }

    #[must_use]
    pub fn attacks(self, square: Square) -> Bitboard {
        table::ATTACKS[self][square]
    }

    const fn attacks_from(self, origin: Bitboard) -> Bitboard {
        match self {
            Leaper::Knight => knight::attacks(origin),
            Leaper::King => king::attacks(origin),
            Leaper::WhitePawn => pawn::attacks(Color::White, origin),
            Leaper::BlackPawn => pawn::attacks(Color::Black, origin),
        }
    }
}
