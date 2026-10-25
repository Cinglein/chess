use std::collections::BTreeSet;

use crate::package_graph::PackageGraph;
use task::CrateName;
use task::SourceFile;

pub struct TestedCrates(BTreeSet<CrateName>);

impl TestedCrates {
    const TEST_ATTRIBUTE: &str = "#[test]";

    #[must_use]
    pub fn collect(files: &[SourceFile]) -> TestedCrates {
        TestedCrates(
            files
                .iter()
                .filter(|file| file.text().contains(Self::TEST_ATTRIBUTE))
                .filter_map(SourceFile::crate_name)
                .filter_map(|name| name.parse().ok())
                .collect(),
        )
    }

    #[must_use]
    pub fn around(
        &self,
        graph: &PackageGraph,
        mutated: &BTreeSet<CrateName>,
    ) -> BTreeSet<CrateName> {
        graph
            .dependents_closure(mutated.clone())
            .into_iter()
            .filter(|name| self.0.contains(name) || mutated.contains(name))
            .collect()
    }
}
