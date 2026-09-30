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
#[repr(u8)]
pub enum Rank {
    #[strum(serialize = "1")]
    One,
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for Rank {
    type Parameters = ();
    type Strategy = proptest::sample::Select<Rank>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        proptest::sample::select(<Rank as strum::VariantArray>::VARIANTS)
    }
}
