use std::collections::BTreeSet;

use affected::TestedCrates;
use task::CrateName;
use task::MutantsArgument;
use task::PackageGraph;
use task::Shard;

pub struct TestPackages(BTreeSet<CrateName>);

impl TestPackages {
    pub fn around(run: &Shard, graph: &PackageGraph, tested: &TestedCrates) -> TestPackages {
        TestPackages(tested.around(graph, &run.crates()))
    }

    pub fn arguments(&self) -> Vec<String> {
        self.0
            .iter()
            .flat_map(|name| [MutantsArgument::TestPackage.to_string(), name.to_string()])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::TestPackages;
    use affected::TestedCrates;
    use task::{PackageGraph, SourceFile};
    const METADATA: &str = r#"{"workspace_root": "/w", "packages": [
        {"name": "board", "manifest_path": "/w/crates/board/Cargo.toml", "dependencies": []},
        {"name": "eval", "manifest_path": "/w/crates/eval/Cargo.toml", "dependencies": [{"name": "board", "path": "/w/crates/board"}]},
        {"name": "search", "manifest_path": "/w/crates/search/Cargo.toml", "dependencies": [{"name": "eval", "path": "/w/crates/eval"}]}
    ]}"#;
    const EVAL: &str = "#[cfg(test)] mod tests { #[test] fn t() {} }";
    const FILES: [(&str, &str, &str); 2] = [
        ("eval", "crates/eval/src/lib.rs", EVAL),
        ("search", "crates/search/src/lib.rs", "pub struct S;"),
    ];
    const ARGUMENTS: [&str; 4] = ["--test-package", "board", "--test-package", "eval"];

    #[test]
    fn a_run_is_tested_by_its_crate_and_by_the_dependents_that_have_tests() {
        let files = FILES.map(|(name, path, text)| {
            SourceFile::parse(path, text.to_owned())
                .unwrap()
                .in_crate(name.parse().unwrap())
        });
        let graph = PackageGraph::from_metadata(METADATA).unwrap();
        let tested = TestedCrates::collect(&files);
        let packages = TestPackages::around(&"board".parse().unwrap(), &graph, &tested);
        assert_eq!(packages.arguments(), ARGUMENTS);
    }
}
