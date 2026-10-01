use std::fs;

use crate::task::failure::Failure;
use crate::task::site::Site;
use crate::task::workspace::Workspace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    ChangedLines,
    Workspace,
}

impl Scope {
    const BASE: &str = "origin/main";
    const DIFF_FILE: &str = "target/mutants.diff";
    const PARALLEL_JOBS: &str = "2";

    pub fn warm_build_directory(self, workspace: &Workspace) -> Result<(), Failure> {
        match self {
            Scope::ChangedLines => Ok(()),
            Scope::Workspace => workspace.cargo(&["test", "--workspace", "--lib", "--no-run"]),
        }
    }

    pub fn arguments(self, workspace: &Workspace) -> Result<Vec<String>, Failure> {
        match self {
            Scope::ChangedLines => {
                let diff = workspace.git_output(&["diff", "--merge-base", Self::BASE])?;
                fs::write(workspace.root().join(Self::DIFF_FILE), diff).map_err(|error| {
                    Failure::Io {
                        site: Site::File(Self::DIFF_FILE.to_owned()),
                        error,
                    }
                })?;
                Ok(vec![
                    String::from("--jobs"),
                    String::from(Self::PARALLEL_JOBS),
                    String::from("--in-diff"),
                    String::from(Self::DIFF_FILE),
                ])
            }
            Scope::Workspace => Ok(vec![String::from("--in-place")]),
        }
    }
}
