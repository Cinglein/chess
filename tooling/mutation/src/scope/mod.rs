mod test_packages;

use std::env;
use std::fs;

use test_packages::TestPackages;

use affected::{Affected, TestedCrates};
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
    const OUTPUT_DIRECTORY: &str = "target/mutants";
    const SHARD_VARIABLE: &str = "MUTANTS_SHARD";

    pub fn applied(
        self,
        line: CommandLine,
        workspace: &Workspace,
        selection: &Selection,
    ) -> Result<Vec<CommandLine>, Failure> {
        let line = line.argument(MutantsArgument::InPlace);
        match self {
            Scope::ChangedLines => {
                Self::write_diff(workspace)?;
                Ok(vec![
                    line.argument(MutantsArgument::Output)
                        .argument(Self::OUTPUT_DIRECTORY)
                        .argument(MutantsArgument::TestWorkspaceOn)
                        .argument(MutantsArgument::InDiff)
                        .argument(Self::DIFF_FILE),
                ])
            }
            Scope::Workspace => match env::var(Self::SHARD_VARIABLE) {
                Ok(spelled) => Self::one_shard(&line, workspace, &spelled.parse()?),
                Err(_) => Ok(vec![
                    line.argument(MutantsArgument::Output)
                        .argument(Self::OUTPUT_DIRECTORY)
                        .argument(MutantsArgument::TestWorkspaceOn)
                        .arguments(selection.arguments::<MutationTool>()),
                ]),
            },
        }
    }

    fn one_shard(
        line: &CommandLine,
        workspace: &Workspace,
        shard: &Shard,
    ) -> Result<Vec<CommandLine>, Failure> {
        let graph = Affected::package_graph(workspace)?;
        let tested = TestedCrates::collect(&workspace.source_files()?);
        shard
            .runs()
            .iter()
            .map(|run| {
                Ok(line
                    .clone()
                    .argument(MutantsArgument::Output)
                    .argument(Self::prepared_output(workspace, run)?)
                    .argument(MutantsArgument::TestWorkspaceOff)
                    .arguments(run.arguments())
                    .arguments(TestPackages::around(run, &graph, &tested).arguments()))
            })
            .collect()
    }

    fn prepared_output(workspace: &Workspace, run: &Shard) -> Result<String, Failure> {
        let crates: Vec<String> = run.crates().iter().map(ToString::to_string).collect();
        let directory = format!("{}/{}", Self::OUTPUT_DIRECTORY, crates.join("+"));
        fs::create_dir_all(workspace.root().join(&directory)).map_err(|error| Failure::Io {
            site: Site::File(directory.clone().into()),
            error,
        })?;
        Ok(directory)
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
