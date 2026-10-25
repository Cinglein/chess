use std::convert::Infallible;
use std::str::FromStr;

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CrateName(String);

impl FromStr for CrateName {
    type Err = Infallible;

    fn from_str(name: &str) -> Result<CrateName, Infallible> {
        Ok(CrateName(name.to_owned()))
    }
}
