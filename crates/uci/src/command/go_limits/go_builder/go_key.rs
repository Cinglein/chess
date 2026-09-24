use strum::EnumString;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
#[repr(u8)]
pub(super) enum GoKey {
    Depth,
    Nodes,
    MoveTime,
    WTime,
    BTime,
    WInc,
    BInc,
}
