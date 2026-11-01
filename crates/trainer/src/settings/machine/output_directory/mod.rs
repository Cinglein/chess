mod error;

use error::OutputDirectoryError;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(try_from = "String")]
pub struct OutputDirectory(String);

impl AsRef<str> for OutputDirectory {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for OutputDirectory {
    type Error = OutputDirectoryError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.is_empty() {
            Err(OutputDirectoryError::Empty)
        } else if text.contains('\0') {
            Err(OutputDirectoryError::Nul)
        } else {
            Ok(Self(text))
        }
    }
}
