mod test_packages;

use std::env;
use std::fs;

use test_packages::TestPackages;

use affected::Affected;
use task::Failure;
use task::Shard;
use task::Site;
use task::Workspace;
use task::{CommandLine, GitArgument, MutantsArgument, Program, Stdout};
use task::{MutationTool, Selection};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    ChangedLines,
    Workspace,
}

impl Scope {
    const BASE: &str = "origin/main";
    const DIFF_FILE: &str = "target/mutants.diff";
    const SHARD_VARIABLE: &str = "MUTANTS_SHARD";

    pub fn applied(
        self,
        line: CommandLine,
        workspace: &Workspace,
        selection: &Selection,
    ) -> Result<CommandLine, Failure> {
        let line = line.argument(MutantsArgument::InPlace);
        match self {
            Scope::ChangedLines => {
                Self::write_diff(workspace)?;
                Ok(line
                    .argument(MutantsArgument::TestWorkspaceOn)
                    .argument(MutantsArgument::InDiff)
                    .argument(Self::DIFF_FILE))
            }
            Scope::Workspace => match env::var(Self::SHARD_VARIABLE) {
                Ok(spelled) => Self::one_shard(line, workspace, &spelled.parse()?),
                Err(_) => Ok(line
                    .argument(MutantsArgument::TestWorkspaceOn)
                    .arguments(selection.arguments::<MutationTool>())),
            },
        }
    }

    fn one_shard(
        line: CommandLine,
        workspace: &Workspace,
        shard: &Shard,
    ) -> Result<CommandLine, Failure> {
        let tests = TestPackages::around(
            shard,
            &Affected::package_graph(workspace)?,
            &workspace.source_files()?,
        );
        Ok(line
            .argument(MutantsArgument::TestWorkspaceOff)
            .arguments(shard.arguments())
            .arguments(tests.arguments()))
    }

    fn write_diff(workspace: &Workspace) -> Result<(), Failure> {
        let diff = workspace
            .line(Program::Git)
            .argument(GitArgument::Diff)
            .argument(GitArgument::MergeBase)
            .argument(Self::BASE)
            .captured::<Stdout>()?;
        fs::write(workspace.root().join(Self::DIFF_FILE), diff).map_err(|error| Failure::Io {
            site: Site::File(Self::DIFF_FILE.into()),
            error,
        })
    }
}
