use std::fs;

use crate::task::failure::Failure;
use crate::task::selection::Selection;
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

    pub fn warm_build_directory(
        self,
        workspace: &Workspace,
        selection: &Selection,
    ) -> Result<(), Failure> {
        match self {
            Scope::ChangedLines => Ok(()),
            Scope::Workspace => {
                let arguments: Vec<String> = [String::from("test")]
                    .into_iter()
                    .chain(selection.package_arguments())
                    .chain([String::from("--lib"), String::from("--no-run")])
                    .collect();
                let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
                workspace.cargo(&borrowed)
            }
        }
    }

    pub fn arguments(
        self,
        workspace: &Workspace,
        selection: &Selection,
    ) -> Result<Vec<String>, Failure> {
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
            Scope::Workspace => Ok([String::from("--in-place")]
                .into_iter()
                .chain(selection.explicit_packages())
                .collect()),
        }
    }
}
