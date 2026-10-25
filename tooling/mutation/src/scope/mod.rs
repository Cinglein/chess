mod mutants_run;
mod output_directory;
mod test_packages;

use std::collections::BTreeSet;
use std::env;
use std::fs;

use affected::{Affected, Changes, Packing, Reach, TestedCrates};
use mutants_run::MutantsRun;
use output_directory::OutputDirectory;
use task::CrateName;
use task::Failure;
use task::Shard;
use task::Site;
use task::Slice;
use task::Workspace;
use task::{CargoArgument, CommandLine, GitArgument, MutantsArgument, Program, Stdout};
use task::{Interval, Policy};
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
        policy: &Policy,
    ) -> Result<Vec<MutantsRun>, Failure> {
        let output = OutputDirectory::cleared(workspace.root())?;
        let line = line.argument(MutantsArgument::InPlace);
        match self {
            Scope::ChangedLines => {
                Self::write_diff(workspace)?;
                let line = line
                    .argument(MutantsArgument::InDiff)
                    .argument(Self::DIFF_FILE);
                Self::changed_lines(line, workspace, &output, policy)
            }
            Scope::Workspace => match env::var(Self::SHARD_VARIABLE) {
                Ok(spelled) => Self::shard_runs(&line, workspace, &output, &[spelled.parse()?]),
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
        policy: &Policy,
    ) -> Result<Vec<MutantsRun>, Failure> {
        let changed: BTreeSet<CrateName> = Changes::SinceMain
            .reaches(workspace)?
            .iter()
            .filter_map(Reach::package)
            .collect();
        let slice = env::var(Self::SLICE_VARIABLE)
            .ok()
            .map(|spelled| spelled.parse::<Slice>())
            .transpose()?;
        if changed.is_empty() {
            let sliced = match slice {
                Some(slice) => line.argument(MutantsArgument::Shard).argument(slice),
                None => line,
            };
            Ok(vec![MutantsRun::new(
                sliced
                    .argument(MutantsArgument::Output)
                    .argument(output.root().display())
                    .argument(MutantsArgument::TestWorkspaceOn),
                output.root().to_path_buf(),
            )])
        } else {
            let listing = workspace
                .line(Program::Cargo)
                .argument(CargoArgument::Mutants)
                .argument(MutantsArgument::InDiff)
                .argument(Self::DIFF_FILE);
            let loads = Affected::loads(listing, &Selection::Crates(changed), policy)?;
            let packed = Packing::shards(&loads, policy.interval(Interval::PerSlice));
            let shards = match slice {
                Some(slice) => slice.picked(packed),
                None => packed,
            };
            let line = line.argument(MutantsArgument::BaselineSkip);
            Self::shard_runs(&line, workspace, output, &shards)
        }
    }

    fn shard_runs(
        line: &CommandLine,
        workspace: &Workspace,
        output: &OutputDirectory,
        shards: &[Shard],
    ) -> Result<Vec<MutantsRun>, Failure> {
        let graph = workspace.packages();
        let tested = TestedCrates::collect(&workspace.source_files()?);
        shards
            .iter()
            .flat_map(Shard::runs)
            .map(|run| {
                let directory = output.prepared_for(&run)?;
                Ok(MutantsRun::new(
                    line.clone()
                        .argument(MutantsArgument::Output)
                        .argument(directory.display())
                        .argument(MutantsArgument::TestWorkspaceOff)
                        .arguments(run.arguments())
                        .arguments(TestPackages::around(&run, graph, &tested).arguments()),
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
