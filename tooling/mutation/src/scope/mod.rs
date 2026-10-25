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
        let output = OutputDirectory::cleared(workspace.root())?;
        let line = line.argument(MutantsArgument::InPlace);
        match self {
            Scope::ChangedLines => {
                Self::write_diff(workspace)?;
                let line = line
                    .argument(MutantsArgument::InDiff)
                    .argument(Self::DIFF_FILE);
                let slice = env::var(Self::SLICE_VARIABLE)
                    .ok()
                    .map(|spelled| spelled.parse::<Slice>())
                    .transpose()?;
                Self::changed_lines(line, workspace, &output, slice)
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

    fn changed_lines(
        line: CommandLine,
        workspace: &Workspace,
        output: &OutputDirectory,
        slice: Option<Slice>,
    ) -> Result<Vec<MutantsRun>, Failure> {
        let reaches = Changes::SinceMain.reaches(workspace)?;
        let changed: BTreeSet<CrateName> = reaches.iter().filter_map(Reach::package).collect();
        let sliced = |line: CommandLine| match slice {
            Some(slice) => line.argument(MutantsArgument::Shard).argument(slice),
            None => line,
        };
        if reaches.contains(&Reach::Everything) || changed.is_empty() {
            Ok(vec![MutantsRun::new(
                sliced(line)
                    .argument(MutantsArgument::Output)
                    .argument(output.root().display())
                    .argument(MutantsArgument::TestWorkspaceOn),
                output.root().to_path_buf(),
            )])
        } else {
            let line = line.argument(MutantsArgument::BaselineSkip);
            match slice.and_then(|slice| slice.picked(changed.iter().cloned().collect())) {
                Some(picked) => Self::one_shard(
                    &line,
                    workspace,
                    output,
                    &Shard::Whole(picked.into_iter().collect()),
                ),
                None => Self::one_shard(&sliced(line), workspace, output, &Shard::Whole(changed)),
            }
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
