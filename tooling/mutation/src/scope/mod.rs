mod mutants_run;
mod test_packages;

use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use affected::{Affected, TestedCrates};
use mutants_run::MutantsRun;
use task::Failure;
use task::Shard;
use task::Site;
use task::Workspace;
use task::{CommandLine, GitArgument, MutantsArgument, Program, Stdout};
use task::{MutationTool, Selection};
use test_packages::TestPackages;

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
    ) -> Result<Vec<MutantsRun>, Failure> {
        let output = Self::cleared_output(workspace)?;
        let line = line.argument(MutantsArgument::InPlace);
        match self {
            Scope::ChangedLines => {
                Self::write_diff(workspace)?;
                Ok(vec![MutantsRun::new(
                    line.argument(MutantsArgument::Output)
                        .argument(Self::OUTPUT_DIRECTORY)
                        .argument(MutantsArgument::TestWorkspaceOn)
                        .argument(MutantsArgument::InDiff)
                        .argument(Self::DIFF_FILE),
                    output,
                )])
            }
            Scope::Workspace => match env::var(Self::SHARD_VARIABLE) {
                Ok(spelled) => Self::one_shard(&line, workspace, &output, &spelled.parse()?),
                Err(_) => Ok(vec![MutantsRun::new(
                    line.argument(MutantsArgument::Output)
                        .argument(Self::OUTPUT_DIRECTORY)
                        .argument(MutantsArgument::TestWorkspaceOn)
                        .arguments(selection.arguments::<MutationTool>()),
                    output,
                )]),
            },
        }
    }

    fn one_shard(
        line: &CommandLine,
        workspace: &Workspace,
        output: &Path,
        shard: &Shard,
    ) -> Result<Vec<MutantsRun>, Failure> {
        let graph = Affected::package_graph(workspace)?;
        let tested = TestedCrates::collect(&workspace.source_files()?);
        shard
            .runs()
            .iter()
            .map(|run| {
                let directory = Self::prepared_output(output, run)?;
                Ok(MutantsRun::new(
                    line.clone()
                        .argument(MutantsArgument::Output)
                        .argument(directory.display())
                        .argument(MutantsArgument::TestWorkspaceOff)
                        .arguments(run.arguments())
                        .arguments(TestPackages::around(run, &graph, &tested).arguments()),
                    directory,
                ))
            })
            .collect()
    }

    fn cleared_output(workspace: &Workspace) -> Result<PathBuf, Failure> {
        let output = workspace.root().join(Self::OUTPUT_DIRECTORY);
        match fs::remove_dir_all(&output) {
            Err(error) if error.kind() != ErrorKind::NotFound => Err(Failure::Io {
                site: Site::File(Self::OUTPUT_DIRECTORY.into()),
                error,
            }),
            _ => Ok(output),
        }
    }

    fn prepared_output(output: &Path, run: &Shard) -> Result<PathBuf, Failure> {
        let crates: Vec<String> = run.crates().iter().map(ToString::to_string).collect();
        let directory = output.join(crates.join("+"));
        fs::create_dir_all(&directory).map_err(|error| Failure::Io {
            site: Site::File(directory.clone()),
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
