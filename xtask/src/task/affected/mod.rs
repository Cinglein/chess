mod package_graph;
mod plan;
mod reach;

use std::collections::BTreeSet;

use package_graph::PackageGraph;
use plan::Plan;
use reach::Reach;

use crate::task::command_line::{CargoWord, GitWord, MutantsWord, Program};
use crate::task::crate_name::CrateName;
use crate::task::failure::Failure;
use crate::task::policy::Policy;
use crate::task::selection::Selection;
use crate::task::workspace::Workspace;

pub struct Affected;

impl Affected {
    const BASE: &str = "origin/main";

    pub fn run(workspace: &Workspace, policy: &Policy) -> Result<(), Failure> {
        let crates = Self::reached(workspace)?;
        let plan = Plan::new(&crates, Self::mutant_count(workspace, &crates)?, policy);
        println!("{}", serde_json::to_string(&plan)?);
        Ok(())
    }

    fn mutant_count(workspace: &Workspace, crates: &BTreeSet<CrateName>) -> Result<usize, Failure> {
        if crates.is_empty() {
            return Ok(0);
        }
        Ok(workspace
            .line(Program::Cargo)
            .word(CargoWord::Mutants)
            .word(MutantsWord::List)
            .words(Selection::Crates(crates.clone()).explicit_packages())
            .stdout()?
            .lines()
            .count())
    }

    pub fn reached(workspace: &Workspace) -> Result<BTreeSet<CrateName>, Failure> {
        let changed = workspace
            .line(Program::Git)
            .word(GitWord::Diff)
            .word(GitWord::NameOnly)
            .word(GitWord::MergeBase)
            .word(Self::BASE)
            .stdout()?;
        let metadata = workspace
            .line(Program::Cargo)
            .word(CargoWord::Metadata)
            .word(CargoWord::FormatVersionOne)
            .word(CargoWord::NoDeps)
            .stdout()?;
        let graph = PackageGraph::from_metadata(&metadata)?;
        Ok(Self::crates(&graph, changed.lines()))
    }

    fn crates<'paths>(
        graph: &PackageGraph,
        changed: impl Iterator<Item = &'paths str>,
    ) -> BTreeSet<CrateName> {
        let reaches: Vec<Reach> = changed.map(Reach::reached_by).collect();
        if reaches.contains(&Reach::Everything) {
            graph.names()
        } else {
            graph.dependents_closure(reaches.iter().filter_map(Reach::package).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Affected, PackageGraph};

    const METADATA: &str = r#"{"packages": [
        {"name": "board", "dependencies": []},
        {"name": "eval", "dependencies": [{"name": "board", "path": "/w/crates/board"}]},
        {"name": "arena", "dependencies": [{"name": "eval", "path": "/w/crates/eval"}]},
        {"name": "tui", "dependencies": []}
    ]}"#;
    const EVAL_CHANGE: [&str; 2] = ["crates/eval/src/lib.rs", "README.md"];
    const ROOT_CHANGE: [&str; 1] = ["Cargo.toml"];

    #[test]
    fn a_crate_change_reaches_its_dependents_and_a_root_change_reaches_every_crate() {
        let graph = PackageGraph::from_metadata(METADATA).unwrap();
        let from_eval = Affected::crates(&graph, EVAL_CHANGE.into_iter());
        let from_root = Affected::crates(&graph, ROOT_CHANGE.into_iter());
        assert_eq!(
            (
                from_eval
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
                from_root.len()
            ),
            (vec!["arena".to_owned(), "eval".to_owned()], 4)
        );
    }
}
