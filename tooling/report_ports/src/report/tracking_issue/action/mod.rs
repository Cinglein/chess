use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, ExitStatus};

use super::issue_number::IssueNumber;
use crate::report::verdict::Verdict;

mod note;

use note::Note;

pub(super) enum Action {
    Close(IssueNumber),
    Refresh(IssueNumber),
    Open,
    Leave,
}

impl Action {
    pub(super) fn choose(verdict: Verdict, existing: Option<IssueNumber>) -> Self {
        match (verdict, existing) {
            (Verdict::Green, Some(number)) => Self::Close(number),
            (Verdict::Green, None) => Self::Leave,
            (Verdict::Red, Some(number)) => Self::Refresh(number),
            (Verdict::Red, None) => Self::Open,
        }
    }

    pub(super) fn carry_out(
        &self,
        title: &OsStr,
        run_url: &OsStr,
        body: &Path,
    ) -> Option<ExitStatus> {
        match self {
            Self::Close(number) => Command::new("gh")
                .args(["issue", "close"])
                .arg(number)
                .arg("--comment")
                .arg(Note::Green.comment_on(run_url))
                .status()
                .ok(),
            Self::Refresh(number) => Command::new("gh")
                .args(["issue", "edit"])
                .arg(number)
                .arg("--body-file")
                .arg(body)
                .status()
                .ok()
                .and_then(|_| {
                    Command::new("gh")
                        .args(["issue", "comment"])
                        .arg(number)
                        .arg("--body")
                        .arg(Note::Failing.comment_on(run_url))
                        .status()
                        .ok()
                }),
            Self::Open => Command::new("gh")
                .args(["issue", "create", "--title"])
                .arg(title)
                .arg("--body-file")
                .arg(body)
                .status()
                .ok(),
            Self::Leave => None,
        }
    }
}
