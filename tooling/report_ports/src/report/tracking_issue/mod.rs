mod action;
mod issue_number;

use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::str;

use action::Action;
use issue_number::IssueNumber;

use super::body::Body;
use super::verdict::Verdict;

pub(super) struct TrackingIssue {
    run_url: OsString,
    title: OsString,
}

impl TrackingIssue {
    const SELECT: &'static str = "first(.[] | select(.title == env.ISSUE_TITLE) | .number)";

    pub(super) const fn new(run_url: OsString, title: OsString) -> Self {
        Self { run_url, title }
    }

    pub(super) fn settle(&self, verdict: Verdict, body: &Body, file: &Path) -> Option<ExitStatus> {
        File::create(file)
            .and_then(|mut written| write!(written, "{body}"))
            .ok()?;
        Action::choose(verdict, self.open_number()).carry_out(&self.title, &self.run_url, file)
    }

    fn open_number(&self) -> Option<IssueNumber> {
        let listed = Command::new("gh")
            .args(["issue", "list", "--state", "open", "--json", "number,title"])
            .args(["--jq", Self::SELECT])
            .env("ISSUE_TITLE", &self.title)
            .output()
            .ok()?;
        str::from_utf8(&listed.stdout).ok()?.parse().ok()
    }
}
