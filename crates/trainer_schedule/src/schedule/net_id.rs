use newtype::Newtype;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Newtype)]
#[serde(transparent)]
pub struct NetId(String);
