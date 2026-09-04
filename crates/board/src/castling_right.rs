use enumset::EnumSetType;
use strum::{Display, EnumString};

use crate::castling_side::CastlingSide;
use crate::color::Color;
use crate::file::File;
use crate::square::Square;

#[derive(Debug, Hash, Display, EnumString, EnumSetType)]
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
    pub const fn new(color: Color, side: CastlingSide) -> CastlingRight {
        match (color, side) {
            (Color::White, CastlingSide::Kingside) => CastlingRight::WhiteKingside,
            (Color::White, CastlingSide::Queenside) => CastlingRight::WhiteQueenside,
            (Color::Black, CastlingSide::Kingside) => CastlingRight::BlackKingside,
            (Color::Black, CastlingSide::Queenside) => CastlingRight::BlackQueenside,
        }
    }

    #[must_use]
    pub const fn color(self) -> Color {
        match self {
            CastlingRight::WhiteKingside | CastlingRight::WhiteQueenside => Color::White,
            CastlingRight::BlackKingside | CastlingRight::BlackQueenside => Color::Black,
        }
    }

    #[must_use]
    pub const fn side(self) -> CastlingSide {
        match self {
            CastlingRight::WhiteKingside | CastlingRight::BlackKingside => CastlingSide::Kingside,
            CastlingRight::WhiteQueenside | CastlingRight::BlackQueenside => {
                CastlingSide::Queenside
            }
        }
    }

    #[must_use]
    pub const fn king_square(self) -> Square {
        Square::new(File::E, self.color().back_rank())
    }

    #[must_use]
    pub const fn king_destination(self) -> Square {
        Square::new(
            self.side().king_destination_file(),
            self.color().back_rank(),
        )
    }

    #[must_use]
    pub const fn rook_square(self) -> Square {
        Square::new(self.side().rook_file(), self.color().back_rank())
    }

    #[must_use]
    pub const fn rook_destination(self) -> Square {
        Square::new(
            self.side().rook_destination_file(),
            self.color().back_rank(),
        )
    }
}
