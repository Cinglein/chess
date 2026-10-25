mod changes;
mod job;
mod job_plan;
mod listed_mutant;
mod package_graph;
mod plan;
mod reach;
mod tool_graph;

pub use changes::Changes;
pub use job::Job;
pub use package_graph::PackageGraph;
pub use plan::Plan;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use job::{ToolScope, ToolTrigger};
use job_plan::JobPlan;
use listed_mutant::ListedMutant;
use reach::Reach;
use strum::IntoEnumIterator;
use tool_graph::ToolGraph;

use task::CrateName;
use task::Failure;
use task::Policy;
use task::Workspace;
use task::{CargoArgument, MutantsArgument, Program, Stdout};
use task::{MutationTool, Selection};

pub struct Affected;

impl Affected {
    const TOOL: &str = "xtask";

    pub fn run(workspace: &Workspace, policy: &Policy, plan: Plan) -> Result<(), Failure> {
        let counts = plan
            .job(Job::Full)
            .map(|full| Self::mutant_counts(workspace, &full.selection()))
            .transpose()?;
        let sharded = match counts {
            Some(counts) => plan.with_full_sharded(&counts, policy),
            None => plan,
        };
        println!("{}", serde_json::to_string(&sharded)?);
        Ok(())
    }

    pub fn plan(workspace: &Workspace, changes: Changes) -> Result<Plan, Failure> {
        let reaches = changes.reaches(workspace)?;
        let graph = Self::package_graph(workspace)?;
        let tool_graph = ToolGraph::from_files(&workspace.source_files()?);
        Ok(Plan::new(Self::jobs(&graph, &tool_graph, &reaches)))
    }

    pub fn package_graph(workspace: &Workspace) -> Result<PackageGraph, Failure> {
        let metadata = workspace
            .line(Program::Cargo)
            .argument(CargoArgument::Metadata)
            .argument(CargoArgument::FormatVersionOne)
            .argument(CargoArgument::NoDeps)
            .captured::<Stdout>()?;
        PackageGraph::from_metadata(&metadata)
    }

    fn jobs(
        graph: &PackageGraph,
        tool_graph: &ToolGraph,
        reaches: &[Reach],
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
                    JobPlan::over(&Self::crates_for(job, touched(job), &product, graph)),
                )
            })
            .collect()
    }

    fn crates_for(
        job: Job,
        touched: bool,
        product: &BTreeSet<CrateName>,
        graph: &PackageGraph,
    ) -> BTreeSet<CrateName> {
        let extra: BTreeSet<CrateName> = match job.tool_scope() {
            ToolScope::PlusTheTool if touched => Self::TOOL.parse().into_iter().collect(),
            ToolScope::Everything if touched => graph.names(),
            ToolScope::ProductOnly | ToolScope::PlusTheTool | ToolScope::Everything => {
                BTreeSet::new()
            }
        };
        product.iter().cloned().chain(extra).collect()
    }

    fn mutant_counts(
        workspace: &Workspace,
        selection: &Selection,
    ) -> Result<BTreeMap<CrateName, usize>, Failure> {
        if selection.is_empty() {
            return Ok(BTreeMap::new());
        }
        ListedMutant::tally(
            &workspace
                .line(Program::Cargo)
                .argument(CargoArgument::Mutants)
                .argument(MutantsArgument::List)
                .argument(MutantsArgument::Json)
                .arguments(selection.arguments::<MutationTool>())
                .captured::<Stdout>()?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Affected, Job, PackageGraph, Reach, ToolGraph};
    use task::SourceFile;

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
        let jobs = Affected::jobs(&graph, &tool_graph, &[Reach::reached_by(LINT_CHANGE)]);
        assert!(RUNS.iter().all(|job| jobs.contains_key(job)));
        assert!(SKIPS.iter().all(|job| !jobs.contains_key(job)));
        assert!(
            jobs[&Job::Clippy]
                .selection()
                .includes(&TOOL.parse().unwrap())
        );
    }
}
