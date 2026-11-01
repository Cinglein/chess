use derive_more::{Display, From, Into};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Display, From, Into, Deserialize)]
#[serde(transparent)]
pub struct RoundCount(usize);
