mod metadata;

use std::collections::{BTreeMap, BTreeSet};

use metadata::Metadata;

use crate::task::failure::Failure;

pub struct PackageGraph {
    dependents: BTreeMap<String, BTreeSet<String>>,
}

impl PackageGraph {
    pub fn from_metadata(json: &str) -> Result<PackageGraph, Failure> {
        let metadata: Metadata = serde_json::from_str(json)?;
        let dependents = metadata.packages().iter().fold(
            BTreeMap::new(),
            |graph: BTreeMap<String, BTreeSet<String>>, package| {
                package
                    .local_dependencies()
                    .fold(graph, |mut graph, dependency| {
                        graph
                            .entry(dependency.to_owned())
                            .or_default()
                            .insert(package.name().to_owned());
                        graph
                    })
            },
        );
        Ok(PackageGraph {
            dependents: metadata
                .packages()
                .iter()
                .map(|package| (package.name().to_owned(), BTreeSet::new()))
                .chain(dependents)
                .fold(BTreeMap::new(), |mut all, (name, direct)| {
                    all.entry(name).or_default().extend(direct);
                    all
                }),
        })
    }

    pub fn names(&self) -> BTreeSet<String> {
        self.dependents.keys().cloned().collect()
    }

    pub fn dependents_closure(&self, seeds: BTreeSet<String>) -> BTreeSet<String> {
        let grown: BTreeSet<String> = seeds
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
        let closure = graph.dependents_closure(BTreeSet::from([BOARD.to_owned()]));
        assert_eq!(
            (closure.len(), graph.names().len(), closure.contains(ENGINE)),
            (CRATES, CRATES, true)
        );
    }
}
