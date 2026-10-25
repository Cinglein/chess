use std::fs;

use crate::task::command_line::{
    CargoArgument, CommandLine, GitArgument, MutantsArgument, Program,
};
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
                .argument(CargoArgument::Test)
                .arguments(selection.package_arguments())
                .argument(CargoArgument::Lib)
                .argument(CargoArgument::NoRun)
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
                    .argument(GitArgument::Diff)
                    .argument(GitArgument::MergeBase)
                    .argument(Self::BASE)
                    .stdout()?;
                fs::write(workspace.root().join(Self::DIFF_FILE), diff).map_err(|error| {
                    Failure::Io {
                        site: Site::File(Self::DIFF_FILE.to_owned()),
                        error,
                    }
                })?;
                Ok(line
                    .argument(MutantsArgument::Jobs)
                    .argument(policy.limit(Limit::ParallelJobs).count())
                    .argument(MutantsArgument::InDiff)
                    .argument(Self::DIFF_FILE))
            }
            Scope::Workspace => Ok(line
                .argument(MutantsArgument::InPlace)
                .arguments(selection.explicit_packages())),
        }
    }
}
