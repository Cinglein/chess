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
#[strum(serialize_all = "lowercase")]
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

impl File {
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::File;

    #[test]
    fn files_display_and_parse_as_lowercase_letters() {
        assert_eq!(File::A.to_string(), "a");
        assert_eq!("h".parse(), Ok(File::H));
        assert!("i".parse::<File>().is_err());
        assert!("A".parse::<File>().is_err());
    }
}
