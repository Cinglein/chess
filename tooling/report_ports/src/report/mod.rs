mod body;
mod branch;
mod missed;
mod shards;
mod tracking_issue;
mod verdict;

use std::env;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use body::Body;
use branch::Branch;
use missed::Missed;
use shards::Shards;
use tracking_issue::TrackingIssue;
use verdict::Verdict;

pub struct Report {
    run_url: OsString,
    title: OsString,
    shards: Shards,
    branch: Branch,
}

impl Report {
    const COLLECTED: &'static str = "shards";
    const BODY: &'static str = "body.md";

    #[must_use]
    pub fn from_env() -> Option<Self> {
        let given: Vec<OsString> = env::args_os().skip(1).collect();
        match given.as_slice() {
            [run_url, title, shards, branch] => Some(Self {
                run_url: run_url.clone(),
                title: title.clone(),
                shards: shards.to_str()?.parse().ok()?,
                branch: branch.to_str()?.parse().ok()?,
            }),
            _ => None,
        }
    }

    #[must_use]
    pub fn run(self) -> ExitCode {
        let missed = Missed::under(Path::new(Self::COLLECTED));
        let verdict = Verdict::from_run(self.shards, &missed);
        let body = Body::new(self.run_url.clone(), missed);
        let _summary = env::var_os("GITHUB_STEP_SUMMARY")
            .and_then(|path| OpenOptions::new().append(true).open(path).ok())
            .and_then(|mut summary| write!(summary, "{body}").ok());
        let _tracked = self.branch.on_main(|| {
            TrackingIssue::new(self.run_url.clone(), self.title.clone()).settle(
                verdict,
                &body,
                Path::new(Self::BODY),
            )
        });
        self.shards.exit()
    }
}
