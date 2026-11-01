use derive_more::{Display, FromStr, Into};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Display, FromStr, Into)]
pub struct ShuffleSeed(u64);
