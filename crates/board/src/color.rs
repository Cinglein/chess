use core::ops::Not;

use enum_map::Enum;
use strum::{Display, EnumCount, EnumIter, EnumString, FromRepr, VariantArray};

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
pub enum Color {
    #[strum(serialize = "w")]
    White,
    #[strum(serialize = "b")]
    Black,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for Color {
    type Parameters = ();
    type Strategy = proptest::sample::Select<Color>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        proptest::sample::select(<Color as strum::VariantArray>::VARIANTS)
    }
}
