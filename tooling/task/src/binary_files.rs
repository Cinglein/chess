use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use syn::{Item, ItemMod};

use crate::source_file::SourceFile;

pub struct BinaryFiles(BTreeSet<PathBuf>);

impl BinaryFiles {
    const BINARY_ROOT: &str = "src/main.rs";

    #[must_use]
    pub fn collect(files: &[SourceFile]) -> BinaryFiles {
        let by_path: BTreeMap<&Path, &SourceFile> =
            files.iter().map(|file| (file.path(), file)).collect();
        let mut members = BTreeSet::new();
        files
            .iter()
            .filter(|file| file.path().ends_with(Self::BINARY_ROOT))
            .for_each(|root| Self::gather(root, &by_path, &mut members));
        BinaryFiles(members)
    }

    #[must_use]
    pub fn contains(&self, path: &Path) -> bool {
        self.0.contains(path)
    }

    fn gather(
        file: &SourceFile,
        by_path: &BTreeMap<&Path, &SourceFile>,
        members: &mut BTreeSet<PathBuf>,
    ) {
        if !members.insert(file.path().to_owned()) {
            return;
        }
        file.syntax()
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(module) => Some(module),
                _ => None,
            })
            .flat_map(|module| Self::declared_files(file.path(), module))
            .filter_map(|candidate| by_path.get(candidate.as_path()).copied())
            .for_each(|child| Self::gather(child, by_path, members));
    }

    fn declared_files(parent: &Path, module: &ItemMod) -> Vec<PathBuf> {
        let directory = if parent.ends_with("main.rs") || parent.ends_with("mod.rs") {
            parent.parent().map(Path::to_path_buf)
        } else {
            parent.with_extension("").into()
        }
        .unwrap_or_default();
        let name = module.ident.to_string();
        vec![
            directory.join(format!("{name}.rs")),
            directory.join(&name).join("mod.rs"),
        ]
    }
}
