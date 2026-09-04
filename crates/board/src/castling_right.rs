use enumset::EnumSetType;
use strum::{Display, EnumString, VariantArray};

use crate::color::Color;
use crate::file::File;
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
    pub const fn king_square(self) -> Square {
        Square::new(File::E, self.color().back_rank())
    }

    #[must_use]
    pub const fn king_destination(self) -> Square {
        let file = match self {
            CastlingRight::WhiteKingside | CastlingRight::BlackKingside => File::G,
            CastlingRight::WhiteQueenside | CastlingRight::BlackQueenside => File::C,
        };
        Square::new(file, self.color().back_rank())
    }

    #[must_use]
    pub const fn rook_square(self) -> Square {
        let file = match self {
            CastlingRight::WhiteKingside | CastlingRight::BlackKingside => File::H,
            CastlingRight::WhiteQueenside | CastlingRight::BlackQueenside => File::A,
        };
        Square::new(file, self.color().back_rank())
    }

    #[must_use]
    pub const fn rook_destination(self) -> Square {
        let file = match self {
            CastlingRight::WhiteKingside | CastlingRight::BlackKingside => File::F,
            CastlingRight::WhiteQueenside | CastlingRight::BlackQueenside => File::D,
        };
        Square::new(file, self.color().back_rank())
    }
}
