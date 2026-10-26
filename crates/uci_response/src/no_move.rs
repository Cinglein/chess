use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
#[repr(u8)]
pub(super) enum NoMove {
    #[strum(serialize = "0000")]
    Zeros,
    #[strum(serialize = "(none)")]
    Parenthesised,
}
