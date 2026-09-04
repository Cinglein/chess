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
pub enum Promotion {
    #[strum(serialize = "n")]
    Knight,
    #[strum(serialize = "b")]
    Bishop,
    #[strum(serialize = "r")]
    Rook,
    #[strum(serialize = "q")]
    Queen,
}

impl Promotion {}

impl From<Promotion> for PieceKind {
    fn from(promotion: Promotion) -> PieceKind {
        match promotion {
            Promotion::Knight => PieceKind::Knight,
            Promotion::Bishop => PieceKind::Bishop,
            Promotion::Rook => PieceKind::Rook,
            Promotion::Queen => PieceKind::Queen,
        }
    }
}
