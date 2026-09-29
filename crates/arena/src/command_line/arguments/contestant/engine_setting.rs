use strum::IntoStaticStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoStaticStr)]
#[repr(u8)]
pub(super) enum EngineSetting {
    #[strum(serialize = "UCI_LimitStrength")]
    LimitStrength,
    #[strum(serialize = "UCI_Elo")]
    Elo,
}
