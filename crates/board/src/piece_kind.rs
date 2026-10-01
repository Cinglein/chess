use enum_map::Enum;
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString, FromRepr, VariantArray};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Enum,
    AsRefStr,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    FromRepr,
    VariantArray,
)]
#[repr(u8)]
#[strum(ascii_case_insensitive)]
pub enum PieceKind {
    #[strum(serialize = "p")]
    Pawn,
    #[strum(serialize = "n")]
    Knight,
    #[strum(serialize = "b")]
    Bishop,
    #[strum(serialize = "r")]
    Rook,
    #[strum(serialize = "q")]
    Queen,
    #[strum(serialize = "k")]
    King,
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for PieceKind {
    type Parameters = ();
    type Strategy = proptest::sample::Select<PieceKind>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        proptest::sample::select(<PieceKind as strum::VariantArray>::VARIANTS)
    }
}
