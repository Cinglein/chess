use std::process::ExitCode;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub(super) enum Shards {
    Finished,
    Failed,
}

impl Shards {
    pub(super) fn exit(self) -> ExitCode {
        match self {
            Self::Finished => ExitCode::SUCCESS,
            Self::Failed => ExitCode::FAILURE,
        }
    }
}

impl FromStr for Shards {
    type Err = ();

    fn from_str(text: &str) -> Result<Self, ()> {
        match text {
            "false" => Ok(Self::Finished),
            "true" => Ok(Self::Failed),
            _ => Err(()),
        }
    }
}
