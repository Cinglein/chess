mod package_graph;
mod reach;

use std::collections::BTreeSet;

use package_graph::PackageGraph;
use reach::Reach;

use crate::task::failure::Failure;
use crate::task::workspace::Workspace;

pub struct Affected;

impl Affected {
    const BASE: &str = "origin/main";

    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        let crates: Vec<String> = Self::reached(workspace)?.into_iter().collect();
        println!("{}", serde_json::to_string(&crates)?);
        Ok(())
    }

    pub fn reached(workspace: &Workspace) -> Result<BTreeSet<String>, Failure> {
        let changed = workspace.git_output(&["diff", "--name-only", "--merge-base", Self::BASE])?;
        let metadata =
            workspace.cargo_stdout(&["metadata", "--format-version", "1", "--no-deps"])?;
        let graph = PackageGraph::from_metadata(&metadata)?;
        Ok(Self::crates(&graph, changed.lines()))
    }

    fn crates<'paths>(
        graph: &PackageGraph,
        changed: impl Iterator<Item = &'paths str>,
    ) -> BTreeSet<String> {
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
            (from_eval.into_iter().collect::<Vec<_>>(), from_root.len()),
            (vec!["arena".to_owned(), "eval".to_owned()], 4)
        );
    }
}
