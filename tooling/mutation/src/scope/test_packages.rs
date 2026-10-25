use std::collections::BTreeSet;

use affected::PackageGraph;
use task::CrateName;
use task::MutantsArgument;
use task::Shard;
use task::SourceFile;

pub struct TestPackages(BTreeSet<CrateName>);

impl TestPackages {
    const TEST_ATTRIBUTE: &str = "#[test]";

    pub fn around(shard: &Shard, graph: &PackageGraph, files: &[SourceFile]) -> TestPackages {
        let tested: BTreeSet<CrateName> = files
            .iter()
            .filter(|file| file.text().contains(Self::TEST_ATTRIBUTE))
            .filter_map(SourceFile::crate_name)
            .filter_map(|name| name.parse().ok())
            .collect();
        let mutated = shard.crates();
        TestPackages(
            graph
                .dependents_closure(mutated.clone())
                .into_iter()
                .filter(|name| tested.contains(name) || mutated.contains(name))
                .collect(),
        )
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
    use affected::PackageGraph;
    use task::SourceFile;

    const METADATA: &str = r#"{"packages": [
        {"name": "board", "dependencies": []},
        {"name": "eval", "dependencies": [{"name": "board", "path": "/w/crates/board"}]},
        {"name": "search", "dependencies": [{"name": "eval", "path": "/w/crates/eval"}]}
    ]}"#;
    const FILES: [(&str, &str); 2] = [
        (
            "crates/eval/src/lib.rs",
            "#[cfg(test)] mod tests { #[test] fn t() {} }",
        ),
        ("crates/search/src/lib.rs", "pub struct S;"),
    ];
    const ARGUMENTS: [&str; 4] = ["--test-package", "board", "--test-package", "eval"];

    #[test]
    fn a_shard_is_tested_by_itself_and_by_the_dependents_that_have_tests() {
        let files = FILES.map(|(path, text)| {
            SourceFile::parse(path.to_owned(), text.to_owned()).expect("valid rust")
        });
        let graph = PackageGraph::from_metadata(METADATA).unwrap();
        let packages = TestPackages::around(&"board".parse().unwrap(), &graph, &files);
        assert_eq!(packages.arguments(), ARGUMENTS);
    }
}
