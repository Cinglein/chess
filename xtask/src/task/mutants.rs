use std::fs;

use crate::task::failure::Failure;
use crate::task::site::Site;
use crate::task::workspace::Workspace;

pub struct Mutants;

impl Mutants {
    const BASE: &str = "origin/main";
    const DIFF_FILE: &str = "target/mutants.diff";
    const JOBS: &str = "2";

    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        let diff = workspace.git_output(&["diff", "--merge-base", Self::BASE])?;
        let path = workspace.root().join(Self::DIFF_FILE);
        fs::write(&path, diff).map_err(|error| Failure::Io {
            site: Site::File(Self::DIFF_FILE.to_owned()),
            error,
        })?;
        workspace.cargo(&[
            "mutants",
            "--in-diff",
            Self::DIFF_FILE,
            "--jobs",
            Self::JOBS,
        ])
    }
}
