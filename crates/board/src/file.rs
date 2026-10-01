use enum_map::Enum;
use strum::{Display, EnumCount, EnumIter, EnumString, FromRepr, VariantArray};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    Enum,
    EnumCount,
    EnumIter,
    EnumString,
    FromRepr,
    VariantArray,
)]
#[strum(serialize_all = "lowercase")]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for File {
    type Parameters = ();
    type Strategy = proptest::sample::Select<File>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        proptest::sample::select(<File as strum::VariantArray>::VARIANTS)
    }
}
