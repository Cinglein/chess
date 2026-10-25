mod metadata;

use std::collections::{BTreeMap, BTreeSet};

use metadata::Metadata;

use task::CrateName;
use task::Failure;

pub struct PackageGraph {
    dependents: BTreeMap<CrateName, BTreeSet<CrateName>>,
}

impl PackageGraph {
    pub fn from_metadata(json: &str) -> Result<PackageGraph, Failure> {
        let metadata: Metadata = serde_json::from_str(json)?;
        let dependents = metadata.packages().iter().fold(
            BTreeMap::new(),
            |graph: BTreeMap<CrateName, BTreeSet<CrateName>>, package| {
                package
                    .local_dependencies()
                    .fold(graph, |mut graph, dependency| {
                        graph
                            .entry(dependency.clone())
                            .or_default()
                            .insert(package.name().clone());
                        graph
                    })
            },
        );
        Ok(PackageGraph {
            dependents: metadata
                .packages()
                .iter()
                .map(|package| (package.name().clone(), BTreeSet::new()))
                .chain(dependents)
                .fold(BTreeMap::new(), |mut all, (name, direct)| {
                    all.entry(name).or_default().extend(direct);
                    all
                }),
        })
    }

    #[must_use]
    pub fn names(&self) -> BTreeSet<CrateName> {
        self.dependents.keys().cloned().collect()
    }

    #[must_use]
    pub fn dependents_closure(&self, seeds: BTreeSet<CrateName>) -> BTreeSet<CrateName> {
        let grown: BTreeSet<CrateName> = seeds
            .iter()
            .flat_map(|seed| self.dependents.get(seed).into_iter().flatten().cloned())
            .chain(seeds.iter().cloned())
            .collect();
        if grown == seeds {
            seeds
        } else {
            self.dependents_closure(grown)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::PackageGraph;

    const METADATA: &str = r#"{"packages": [
        {"name": "board", "dependencies": [{"name": "serde", "path": null}]},
        {"name": "search", "dependencies": [{"name": "board", "path": "/w/crates/board"}]},
        {"name": "engine", "dependencies": [{"name": "search", "path": "/w/crates/search"}]}
    ]}"#;
    const BOARD: &str = "board";
    const ENGINE: &str = "engine";
    const CRATES: usize = 3;

    #[test]
    fn the_closure_of_a_crate_is_every_crate_that_depends_on_it_transitively_and_itself() {
        let graph = PackageGraph::from_metadata(METADATA).unwrap();
        let closure = graph.dependents_closure(BTreeSet::from([BOARD.parse().unwrap()]));
        assert_eq!(
            (
                closure.len(),
                graph.names().len(),
                closure.contains(&ENGINE.parse().unwrap())
            ),
            (CRATES, CRATES, true)
        );
    }
}
