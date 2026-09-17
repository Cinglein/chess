use enum_map::Enum;
use strum::{Display, EnumCount, EnumIter, EnumString, FromRepr, VariantArray};

use crate::piece_kind::PieceKind;

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
#[strum(ascii_case_insensitive)]
pub enum PromotionPiece {
    #[strum(serialize = "n")]
    Knight,
    #[strum(serialize = "b")]
    Bishop,
    #[strum(serialize = "r")]
    Rook,
    #[strum(serialize = "q")]
    Queen,
}

impl PromotionPiece {}

impl From<PromotionPiece> for PieceKind {
    fn from(promotion: PromotionPiece) -> PieceKind {
        match promotion {
            PromotionPiece::Knight => PieceKind::Knight,
            PromotionPiece::Bishop => PieceKind::Bishop,
            PromotionPiece::Rook => PieceKind::Rook,
            PromotionPiece::Queen => PieceKind::Queen,
        }
    }
}
