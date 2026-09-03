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
