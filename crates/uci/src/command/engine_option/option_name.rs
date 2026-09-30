use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
#[repr(u8)]
pub(super) enum OptionName {
    #[strum(serialize = "UCI_LimitStrength")]
    LimitStrength,
    #[strum(serialize = "UCI_Elo")]
    Elo,
}
