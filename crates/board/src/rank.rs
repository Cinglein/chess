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
    EnumCount,
    EnumIter,
    EnumString,
    FromRepr,
    VariantArray,
)]
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

impl Rank {
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::Rank;

    #[test]
    fn ranks_display_and_parse_as_digits() {
        assert_eq!(Rank::One.to_string(), "1");
        assert_eq!("8".parse(), Ok(Rank::Eight));
        assert!("9".parse::<Rank>().is_err());
        assert!("0".parse::<Rank>().is_err());
    }
}
