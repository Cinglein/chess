mod module_file;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use module_file::ModuleFile;

use task::SourceFile;
use task::UseEdges;

pub struct ToolGraph {
    dependencies: BTreeMap<PathBuf, BTreeSet<PathBuf>>,
}

impl ToolGraph {
    const TOOL: &str = "xtask/src";

    pub fn from_files(files: &[SourceFile]) -> ToolGraph {
        let known: BTreeSet<&Path> = files
            .iter()
            .map(SourceFile::path)
            .filter(|path| path.starts_with(Self::TOOL))
            .collect();
        ToolGraph {
            dependencies: files
                .iter()
                .filter(|file| known.contains(file.path()))
                .map(|file| {
                    let module = ModuleFile::enclosing(file.path());
                    let edges = UseEdges::in_file(file.syntax())
                        .iter()
                        .filter_map(|path| module.resolve(path, &known))
                        .chain(module.declared_children(file.syntax(), &known))
                        .collect();
                    (file.path().to_path_buf(), edges)
                })
                .collect(),
        }
    }

    pub fn closure<'entries>(
        &self,
        entries: impl Iterator<Item = &'entries str>,
    ) -> BTreeSet<PathBuf> {
        self.grown(entries.map(PathBuf::from).collect())
    }

    fn grown(&self, seeds: BTreeSet<PathBuf>) -> BTreeSet<PathBuf> {
        let grown: BTreeSet<PathBuf> = seeds
            .iter()
            .flat_map(|seed| self.dependencies.get(seed).into_iter().flatten().cloned())
            .chain(seeds.iter().cloned())
            .collect();
        if grown == seeds {
            seeds
        } else {
            self.grown(grown)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ToolGraph;
    use task::SourceFile;

    const LINT: &str = "use crate::task::fn_shape::FnShape; pub struct Lint;";
    const FN_SHAPE: &str = "use crate::task::report::Report; pub struct FnShape;";
    const REPORT: &str = "pub struct Report;";
    const MUTANTS: &str = "mod scope; use scope::Scope; pub struct Mutants;";
    const SCOPE: &str = "use super::Mutants; pub struct Scope;";
    const FILES: [(&str, &str); 5] = [
        ("xtask/src/task/lint.rs", LINT),
        ("xtask/src/task/fn_shape.rs", FN_SHAPE),
        ("xtask/src/task/report.rs", REPORT),
        ("xtask/src/task/mutants/mod.rs", MUTANTS),
        ("xtask/src/task/mutants/scope.rs", SCOPE),
    ];

    const LINT_ENTRY: [&str; 1] = ["xtask/src/task/lint.rs"];
    const MUTANTS_ENTRY: [&str; 1] = ["xtask/src/task/mutants/mod.rs"];
    const LINT_REACH: usize = 3;
    const MUTANTS_REACH: usize = 2;

    #[test]
    fn a_job_entry_reaches_what_it_uses_transitively_and_its_declared_children() {
        let files = FILES.map(|(path, text)| {
            SourceFile::parse(path.to_owned(), text.to_owned()).expect("valid rust")
        });
        let graph = ToolGraph::from_files(&files);
        let lint = graph.closure(LINT_ENTRY.into_iter());
        let mutants = graph.closure(MUTANTS_ENTRY.into_iter());
        assert_eq!((lint.len(), mutants.len()), (LINT_REACH, MUTANTS_REACH));
        assert!(!lint.iter().any(|path| path.ends_with("scope.rs")));
    }
}
