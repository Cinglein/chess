mod job;
mod job_plan;
mod package_graph;
mod plan;
mod reach;
mod tool_graph;

pub use job::{Job, ToolScope, ToolTrigger};
pub use plan::Plan;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use job_plan::JobPlan;
use package_graph::PackageGraph;
use reach::Reach;
use strum::IntoEnumIterator;
use tool_graph::ToolGraph;

use crate::task::command_line::{CargoArgument, GitArgument, MutantsArgument, Program, Stdout};
use crate::task::crate_name::CrateName;
use crate::task::failure::Failure;
use crate::task::policy::Policy;
use crate::task::selection::{MutationTool, Selection};
use crate::task::workspace::Workspace;

pub struct Affected;

impl Affected {
    const BASE: &str = "origin/main";
    const TOOL: &str = "xtask";

    pub fn run(workspace: &Workspace, policy: &Policy) -> Result<(), Failure> {
        let plan = Self::plan(workspace, policy)?;
        let full = plan
            .job(Job::Full)
            .map(|full| Self::mutant_count(workspace, &full.selection()))
            .transpose()?;
        let sharded = match full {
            Some(mutants) => plan.with_full_sharded(mutants, policy),
            None => plan,
        };
        println!("{}", serde_json::to_string(&sharded)?);
        Ok(())
    }

    pub fn plan(workspace: &Workspace, policy: &Policy) -> Result<Plan, Failure> {
        let changed = workspace
            .line(Program::Git)
            .argument(GitArgument::Diff)
            .argument(GitArgument::NameOnly)
            .argument(GitArgument::MergeBase)
            .argument(Self::BASE)
            .captured::<Stdout>()?;
        let metadata = workspace
            .line(Program::Cargo)
            .argument(CargoArgument::Metadata)
            .argument(CargoArgument::FormatVersionOne)
            .argument(CargoArgument::NoDeps)
            .captured::<Stdout>()?;
        let reaches: Vec<Reach> = changed.lines().map(Reach::reached_by).collect();
        let graph = PackageGraph::from_metadata(&metadata)?;
        let tool_graph = ToolGraph::from_files(&workspace.source_files()?);
        Ok(Plan::new(Self::jobs(&graph, &tool_graph, &reaches, policy)))
    }

    fn jobs(
        graph: &PackageGraph,
        tool_graph: &ToolGraph,
        reaches: &[Reach],
        policy: &Policy,
    ) -> BTreeMap<Job, JobPlan> {
        let everything = || reaches.contains(&Reach::Everything);
        let product = if everything() {
            graph.names()
        } else {
            graph.dependents_closure(reaches.iter().filter_map(Reach::package).collect())
        };
        let tool_files: BTreeSet<PathBuf> = reaches
            .iter()
            .filter_map(Reach::tool_file)
            .map(|path| ToolGraph::embedded(path).to_path_buf())
            .collect();
        let touched = |job: Job| {
            everything()
                || reaches
                    .iter()
                    .any(|reach| reach.workflow() == Some(job.workflow()))
                || (job.tool_trigger() == ToolTrigger::AnyToolChange && !tool_files.is_empty())
                || tool_graph
                    .closure(job.entries())
                    .iter()
                    .any(|reached| tool_files.contains(reached))
        };
        Job::iter()
            .filter(|job| touched(*job) || !product.is_empty())
            .map(|job| {
                (
                    job,
                    JobPlan::over(&Self::crates_for(job, touched(job), &product, policy)),
                )
            })
            .collect()
    }

    fn crates_for(
        job: Job,
        touched: bool,
        product: &BTreeSet<CrateName>,
        policy: &Policy,
    ) -> BTreeSet<CrateName> {
        let extra: BTreeSet<CrateName> = match job.tool_scope() {
            ToolScope::PlusTheTool if touched => Self::TOOL.parse().into_iter().collect(),
            ToolScope::SmokeCrate if touched => BTreeSet::from([policy.smoke_crate().clone()]),
            ToolScope::ProductOnly | ToolScope::PlusTheTool | ToolScope::SmokeCrate => {
                BTreeSet::new()
            }
        };
        product.iter().cloned().chain(extra).collect()
    }

    fn mutant_count(workspace: &Workspace, selection: &Selection) -> Result<usize, Failure> {
        if selection.is_empty() {
            return Ok(0);
        }
        Ok(workspace
            .line(Program::Cargo)
            .argument(CargoArgument::Mutants)
            .argument(MutantsArgument::List)
            .arguments(selection.arguments::<MutationTool>())
            .captured::<Stdout>()?
            .lines()
            .count())
    }
}

#[cfg(test)]
mod tests {
    use settings::EmbeddedDefaults;

    use super::{Affected, Job, PackageGraph, Reach, ToolGraph};
    use crate::task::policy::Policy;
    use crate::task::source_file::SourceFile;

    const METADATA: &str = r#"{"packages": [
        {"name": "board", "dependencies": []},
        {"name": "eval", "dependencies": [{"name": "board", "path": "/w/crates/board"}]},
        {"name": "xtask", "dependencies": []}
    ]}"#;
    const LINT: &str = "use crate::task::fn_shape::FnShape; pub struct Lint;";
    const FILES: [(&str, &str); 2] = [
        ("xtask/src/task/lint.rs", LINT),
        ("xtask/src/task/fn_shape.rs", "pub struct FnShape;"),
    ];
    const LINT_CHANGE: &str = "xtask/src/task/fn_shape.rs";
    const SHARD_WORKFLOW: &str = ".github/workflows/mutants.yml";
    const TOOL: &str = "xtask";
    const RUNS: [Job; 2] = [Job::Lint, Job::Clippy];
    const SKIPS: [Job; 2] = [Job::Full, Job::Mutants];

    #[test]
    fn a_lint_change_reaches_the_lint_job_and_the_tools_own_checks_but_not_the_mutant_shards() {
        let graph = PackageGraph::from_metadata(METADATA).unwrap();
        let files = FILES.map(|(path, text)| {
            SourceFile::parse(path.to_owned(), text.to_owned()).expect("valid rust")
        });
        let tool_graph = ToolGraph::from_files(&files);
        let policy = Policy::defaults();
        let jobs = Affected::jobs(
            &graph,
            &tool_graph,
            &[Reach::reached_by(LINT_CHANGE)],
            &policy,
        );
        assert!(RUNS.iter().all(|job| jobs.contains_key(job)));
        assert!(SKIPS.iter().all(|job| !jobs.contains_key(job)));
        assert!(
            jobs[&Job::Clippy]
                .selection()
                .includes(&TOOL.parse().unwrap())
        );
    }

    #[test]
    fn a_shard_workflow_change_reaches_only_the_full_job_over_the_smoke_crate() {
        let graph = PackageGraph::from_metadata(METADATA).unwrap();
        let policy = Policy::defaults();
        let jobs = Affected::jobs(
            &graph,
            &ToolGraph::from_files(&[]),
            &[Reach::reached_by(SHARD_WORKFLOW)],
            &policy,
        );
        assert_eq!(jobs.keys().copied().collect::<Vec<Job>>(), vec![Job::Full]);
        assert!(jobs[&Job::Full].selection().includes(policy.smoke_crate()));
    }
}
