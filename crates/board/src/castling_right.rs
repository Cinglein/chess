use enumset::EnumSetType;
use strum::{Display, EnumString};

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
