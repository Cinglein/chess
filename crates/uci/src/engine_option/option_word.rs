use strum::{Display, IntoStaticStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, IntoStaticStr)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum OptionWord {
    Name,
    Value,
}
