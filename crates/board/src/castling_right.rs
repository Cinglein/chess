use enumset::EnumSetType;
use strum::{Display, EnumString, VariantArray};

use crate::color::Color;

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
}
