use std::fs;

use crate::task::command_line::{CargoWord, CommandLine, GitWord, MutantsWord, Program};
use crate::task::failure::Failure;
use crate::task::policy::{Limit, Policy};
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

    pub fn warm_build_directory(
        self,
        workspace: &Workspace,
        selection: &Selection,
    ) -> Result<(), Failure> {
        match self {
            Scope::ChangedLines => Ok(()),
            Scope::Workspace => workspace
                .line(Program::Cargo)
                .word(CargoWord::Test)
                .words(selection.package_words())
                .word(CargoWord::Lib)
                .word(CargoWord::NoRun)
                .run(),
        }
    }

    pub fn applied(
        self,
        line: CommandLine,
        workspace: &Workspace,
        selection: &Selection,
        policy: &Policy,
    ) -> Result<CommandLine, Failure> {
        match self {
            Scope::ChangedLines => {
                let diff = workspace
                    .line(Program::Git)
                    .word(GitWord::Diff)
                    .word(GitWord::MergeBase)
                    .word(Self::BASE)
                    .stdout()?;
                fs::write(workspace.root().join(Self::DIFF_FILE), diff).map_err(|error| {
                    Failure::Io {
                        site: Site::File(Self::DIFF_FILE.to_owned()),
                        error,
                    }
                })?;
                Ok(line
                    .word(MutantsWord::Jobs)
                    .word(policy.limit(Limit::ParallelJobs).count())
                    .word(MutantsWord::InDiff)
                    .word(Self::DIFF_FILE))
            }
            Scope::Workspace => Ok(line
                .word(MutantsWord::InPlace)
                .words(selection.explicit_packages())),
        }
    }
}
