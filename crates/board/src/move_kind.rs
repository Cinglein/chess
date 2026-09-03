use enum_map::Enum;
use strum::{EnumCount, EnumIter, FromRepr, VariantArray};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Enum, EnumCount, EnumIter, FromRepr, VariantArray,
)]
#[repr(u8)]
pub enum MoveKind {
    Normal,
    Promotion,
    EnPassant,
    Castling,
}

impl MoveKind {
    pub(crate) const fn from_bits(bits: u8) -> MoveKind {
        Self::VARIANTS[bits as usize]
    }

    pub(crate) const fn into_bits(self) -> u8 {
        self as u8
    }
}
