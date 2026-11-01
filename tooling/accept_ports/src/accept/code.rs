use std::process::{ExitCode, ExitStatus};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct Code(i32);

impl Code {
    pub(super) const FAILED: Self = Self(1);

    pub(super) fn exit(self) -> ExitCode {
        u8::try_from(self.0).map_or(ExitCode::FAILURE, ExitCode::from)
    }
}

impl From<ExitStatus> for Code {
    fn from(status: ExitStatus) -> Self {
        status.code().map_or(Self::FAILED, Self)
    }
}

impl FromStr for Code {
    type Err = std::num::ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.parse().map(Self)
    }
}
