use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum CommandWord {
    Uci,
    IsReady,
    SetOption,
    UciNewGame,
    Position,
    Go,
    Stop,
    Quit,
}
