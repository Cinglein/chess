use strum::EnumString;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString)]
#[repr(u8)]
#[strum(serialize_all = "kebab-case")]
pub(super) enum ContestantWord {
    InProcess,
}
