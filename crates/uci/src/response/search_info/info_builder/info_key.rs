use strum::EnumString;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum InfoKey {
    Depth,
    Cp,
    Mate,
    Nodes,
    Time,
    Pv,
}
