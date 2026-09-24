use strum::{Display, EnumString, IntoStaticStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, IntoStaticStr)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum PositionWord {
    StartPos,
    Fen,
    Moves,
}
