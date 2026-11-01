use newtype::Newtype;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize, Newtype)]
#[serde(transparent)]
pub struct NetId(String);
