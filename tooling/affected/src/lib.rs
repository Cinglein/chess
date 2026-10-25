mod changes;
mod crate_load;
mod job;
mod job_plan;
mod listed_mutant;
mod plan;
mod reach;
mod tested_crates;

pub use changes::Changes;
pub use job::Job;
pub use plan::Plan;
pub use reach::Reach;
pub use tested_crates::TestedCrates;

use std::collections::{BTreeMap, BTreeSet};

use crate_load::CrateLoad;
use job::ToolScope;
use job_plan::JobPlan;
use listed_mutant::ListedMutant;
use strum::IntoEnumIterator;

use task::CrateName;
use task::Failure;
use task::PackageGraph;
use task::Policy;
use task::Workspace;
use task::{CargoArgument, MutantsArgument, Program, Stdout};
use task::{MutationTool, Selection};

pub struct Affected;

impl Affected {
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
        Ok(Plan::new(Self::jobs(
            workspace.packages(),
            workspace.tool(),
            &reaches,
        )))
    }

    fn jobs(graph: &PackageGraph, tool: &CrateName, reaches: &[Reach]) -> BTreeMap<Job, JobPlan> {
        let changed: BTreeSet<CrateName> = reaches.iter().filter_map(Reach::package).collect();
        let product = if reaches.contains(&Reach::Everything) {
            graph.names()
        } else {
            graph.dependents_closure(changed.clone())
        };
        let whole = |job: Job| {
            reaches.contains(&Reach::Everything)
                || reaches.contains(&Reach::Driven(job.driver()))
                || (job.tool_scope() == ToolScope::Everything && changed.contains(tool))
        };
        Job::iter()
            .filter(|job| whole(*job) || !product.is_empty())
            .map(|job| {
                let crates = if whole(job) {
                    graph.names()
                } else {
                    product.clone()
                };
                (job, JobPlan::over(&crates))
            })
            .collect()
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
    use super::{Affected, Job, JobPlan, Reach};
    use std::collections::BTreeMap;
    use task::PackageGraph;
    const METADATA: &str = r#"{"workspace_root": "/w", "packages": [
        {"name": "board", "manifest_path": "/w/crates/board/Cargo.toml", "dependencies": []},
        {"name": "eval", "manifest_path": "/w/crates/eval/Cargo.toml", "dependencies": [{"name": "board", "path": "/w/crates/board"}]},
        {"name": "lint", "manifest_path": "/w/tooling/lint/Cargo.toml", "dependencies": []},
        {"name": "xtask", "manifest_path": "/w/xtask/Cargo.toml", "dependencies": [{"name": "lint", "path": "/w/tooling/lint"}]}
    ]}"#;
    const TOOL: &str = "xtask";
    const LINT: &str = "lint";
    const BOARD: &str = "board";
    const EVAL: &str = "eval";
    const RUNS: usize = 2;

    #[test]
    fn a_change_reaches_its_dependents_the_tool_widens_full_and_mutation_config_its_runs() {
        let graph = PackageGraph::from_metadata(METADATA).unwrap();
        let tool = TOOL.parse().unwrap();
        let plan = |reach: Reach| Affected::jobs(&graph, &tool, &[reach]);
        let package = |name: &str| plan(Reach::Package(name.parse().unwrap()));
        let has = |jobs: &BTreeMap<Job, JobPlan>, job: Job, name: &str| {
            jobs[&job].selection().includes(&name.parse().unwrap())
        };
        let (full, clippy) = (Job::Full, Job::Clippy);
        let (lint, tooled, board) = (package(LINT), package(TOOL), package(BOARD));
        let config = plan(Reach::Driven(full.driver()));
        assert!(has(&lint, clippy, TOOL) && !has(&lint, full, EVAL) && !has(&board, clippy, TOOL));
        assert!(has(&tooled, full, EVAL) && has(&board, full, EVAL) && !has(&board, full, LINT));
        assert!(config.len() == RUNS && has(&config, full, EVAL) && !config.contains_key(&clippy));
    }
}
