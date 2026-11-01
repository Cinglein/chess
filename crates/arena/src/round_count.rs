use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Display, From, Into, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct RoundCount(usize);
