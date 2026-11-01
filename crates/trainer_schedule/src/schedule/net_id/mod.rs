mod error;

use derive_more::Into;
use error::NetIdError;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Into, Deserialize)]
#[serde(try_from = "String")]
pub struct NetId(String);

impl TryFrom<String> for NetId {
    type Error = NetIdError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.is_empty() {
            Err(NetIdError::Empty)
        } else if text
            .chars()
            .any(|letter| !letter.is_ascii_alphanumeric() && !matches!(letter, '_' | '-'))
        {
            Err(NetIdError::Character)
        } else {
            Ok(Self(text))
        }
    }
}
