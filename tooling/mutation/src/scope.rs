use std::fs;

use task::Failure;
use task::Site;
use task::Workspace;
use task::{CargoArgument, CommandLine, GitArgument, MutantsArgument, Program, Stdout};
use task::{CargoTool, MutationTool, Selection};
use task::{Limit, Policy};

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
                .arguments(selection.arguments::<CargoTool>())
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
                    .captured::<Stdout>()?;
                fs::write(workspace.root().join(Self::DIFF_FILE), diff).map_err(|error| {
                    Failure::Io {
                        site: Site::File(Self::DIFF_FILE.into()),
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
                .arguments(selection.arguments::<MutationTool>())),
        }
    }
}
