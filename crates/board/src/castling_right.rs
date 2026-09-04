use enumset::EnumSetType;
use strum::{Display, EnumString, VariantArray};

use crate::castling::Castling;
use crate::color::Color;
use crate::square::Square;

#[derive(Debug, Hash, Display, EnumString, EnumSetType, VariantArray)]
pub enum CastlingRight {
    #[strum(serialize = "K")]
    WhiteKingside,
    #[strum(serialize = "Q")]
    WhiteQueenside,
    #[strum(serialize = "k")]
    BlackKingside,
    #[strum(serialize = "q")]
    BlackQueenside,
}

impl CastlingRight {
    #[must_use]
    pub const fn color(self) -> Color {
        match self {
            CastlingRight::WhiteKingside | CastlingRight::WhiteQueenside => Color::White,
            CastlingRight::BlackKingside | CastlingRight::BlackQueenside => Color::Black,
        }
    }

    #[must_use]
    pub const fn castling(self) -> Castling {
        let (king_from, king_to, rook_from, rook_to) = match self {
            CastlingRight::WhiteKingside => (Square::E1, Square::G1, Square::H1, Square::F1),
            CastlingRight::WhiteQueenside => (Square::E1, Square::C1, Square::A1, Square::D1),
            CastlingRight::BlackKingside => (Square::E8, Square::G8, Square::H8, Square::F8),
            CastlingRight::BlackQueenside => (Square::E8, Square::C8, Square::A8, Square::D8),
        };
        Castling {
            king_from,
            king_to,
            rook_from,
            rook_to,
        }
    }
}
