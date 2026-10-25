mod changes;
mod crate_load;
mod job;
mod job_plan;
mod listed_mutant;
mod package_graph;
mod plan;
mod reach;
mod tested_crates;
mod tool_graph;

pub use changes::Changes;
pub use job::Job;
pub use package_graph::PackageGraph;
pub use plan::Plan;
pub use tested_crates::TestedCrates;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate_load::CrateLoad;
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
        let loads = plan
            .job(Job::Full)
            .map(|full| Self::loads(workspace, &full.selection(), policy))
            .transpose()?;
        let sharded = match loads {
            Some(loads) => plan.with_full_sharded(&loads, policy),
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
            .map(Path::to_path_buf)
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

    fn loads(
        workspace: &Workspace,
        selection: &Selection,
        policy: &Policy,
    ) -> Result<Vec<CrateLoad>, Failure> {
        if selection.is_empty() {
            Ok(Vec::new())
        } else {
            let counts = ListedMutant::tally(
                &workspace
                    .line(Program::Cargo)
                    .argument(CargoArgument::Mutants)
                    .argument(MutantsArgument::List)
                    .argument(MutantsArgument::Json)
                    .arguments(selection.arguments::<MutationTool>())
                    .captured::<Stdout>()?,
            )?;
            Ok(counts
                .into_iter()
                .map(|(name, mutants)| {
                    let each = policy.mutant_millis().for_crate(&name);
                    CrateLoad::new(name, mutants, each, policy)
                })
                .collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{Affected, Job, JobPlan, PackageGraph, Reach, ToolGraph};
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
    const TOOL_ROOT: &str = "xtask/src/main.rs";
    const BOARD_FILE: &str = "crates/board/src/lib.rs";
    const EVAL: &str = "eval";

    #[test]
    fn changes_reach_the_jobs_and_crates_their_tool_scope_says_and_no_others() {
        let graph = PackageGraph::from_metadata(METADATA).unwrap();
        let files = FILES.map(|(path, text)| SourceFile::parse(path, text.to_owned()).unwrap());
        let tool_graph = ToolGraph::from_files(&files);
        let plan = |reach: &str| Affected::jobs(&graph, &tool_graph, &[Reach::reached_by(reach)]);
        let (lint, tool, board) = (plan(LINT_CHANGE), plan(TOOL_ROOT), plan(BOARD_FILE));
        let has = |jobs: &BTreeMap<Job, JobPlan>, job: Job, name: &str| {
            jobs[&job].selection().includes(&name.parse().unwrap())
        };
        let (full, clippy) = (Job::Full, Job::Clippy);
        let lint_jobs = RUNS.iter().all(|job| lint.contains_key(job))
            && SKIPS.iter().all(|job| !lint.contains_key(job));
        assert!(lint_jobs && has(&lint, clippy, TOOL));
        assert!(has(&tool, full, EVAL) && has(&tool, clippy, TOOL));
        assert!(!has(&board, clippy, TOOL) && has(&board, full, EVAL) && !has(&board, full, TOOL));
    }
}
