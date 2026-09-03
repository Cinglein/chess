use enum_map::Enum;
use strum::{EnumCount, EnumIter, VariantArray};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Enum, EnumCount, EnumIter, VariantArray)]
#[repr(u8)]
pub enum Diagonal {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
