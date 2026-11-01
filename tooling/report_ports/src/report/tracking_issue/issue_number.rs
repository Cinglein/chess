use std::ffi::OsStr;
use std::str::FromStr;

pub(super) struct IssueNumber(String);

impl AsRef<OsStr> for IssueNumber {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(&self.0)
    }
}

impl FromStr for IssueNumber {
    type Err = std::num::ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.trim()
            .parse::<u64>()
            .map(|_| Self(text.trim().to_owned()))
    }
}
