mod mutants_run;
mod output_directory;
mod test_packages;

use std::collections::BTreeSet;
use std::env;
use std::fs;

use affected::{Affected, Changes, Reach, TestedCrates};
use mutants_run::MutantsRun;
use output_directory::OutputDirectory;
use task::CrateName;
use task::Failure;
use task::Shard;
use task::Site;
use task::Slice;
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
    const SHARD_VARIABLE: &str = "MUTANTS_SHARD";
    const SLICE_VARIABLE: &str = "MUTANTS_SLICE";

    pub fn applied(
        self,
        line: CommandLine,
        workspace: &Workspace,
        selection: &Selection,
    ) -> Result<Vec<MutantsRun>, Failure> {
        let output = OutputDirectory::cleared(workspace)?;
        let line = line.argument(MutantsArgument::InPlace);
        match self {
            Scope::ChangedLines => {
                Self::write_diff(workspace)?;
                let line = line
                    .argument(MutantsArgument::Output)
                    .argument(output.root().display())
                    .argument(MutantsArgument::InDiff)
                    .argument(Self::DIFF_FILE);
                let line = match env::var(Self::SLICE_VARIABLE) {
                    Ok(spelled) => line
                        .argument(MutantsArgument::Shard)
                        .argument(spelled.parse::<Slice>()?),
                    Err(_) => line,
                };
                Ok(vec![MutantsRun::new(
                    Self::tested_by_changes(line, workspace)?,
                    output.root().to_path_buf(),
                )])
            }
            Scope::Workspace => match env::var(Self::SHARD_VARIABLE) {
                Ok(spelled) => Self::one_shard(&line, workspace, &output, &spelled.parse()?),
                Err(_) => Ok(vec![MutantsRun::new(
                    line.argument(MutantsArgument::Output)
                        .argument(output.root().display())
                        .argument(MutantsArgument::TestWorkspaceOn)
                        .arguments(selection.arguments::<MutationTool>()),
                    output.root().to_path_buf(),
                )]),
            },
        }
    }

    fn one_shard(
        line: &CommandLine,
        workspace: &Workspace,
        output: &OutputDirectory,
        shard: &Shard,
    ) -> Result<Vec<MutantsRun>, Failure> {
        let graph = Affected::package_graph(workspace)?;
        let tested = TestedCrates::collect(&workspace.source_files()?);
        shard
            .runs()
            .iter()
            .map(|run| {
                let directory = output.prepared_for(run)?;
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

    fn tested_by_changes(line: CommandLine, workspace: &Workspace) -> Result<CommandLine, Failure> {
        let reaches = Changes::SinceMain.reaches(workspace)?;
        let changed: BTreeSet<CrateName> = reaches.iter().filter_map(Reach::package).collect();
        if reaches.contains(&Reach::Everything) || changed.is_empty() {
            Ok(line.argument(MutantsArgument::TestWorkspaceOn))
        } else {
            let graph = Affected::package_graph(workspace)?;
            let tested = TestedCrates::collect(&workspace.source_files()?);
            let packages = TestPackages::around(&Shard::Whole(changed), &graph, &tested);
            Ok(line
                .argument(MutantsArgument::TestWorkspaceOff)
                .arguments(packages.arguments()))
        }
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
