use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum GoKey {
    Infinite,
    Depth,
    Nodes,
    MoveTime,
    WTime,
    BTime,
    WInc,
    BInc,
}
