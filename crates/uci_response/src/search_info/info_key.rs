use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum InfoKey {
    Depth,
    Score,
    Cp,
    Mate,
    Nodes,
    Nps,
    Time,
    Pv,
    String,
}
