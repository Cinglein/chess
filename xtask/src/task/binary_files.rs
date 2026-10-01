use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use syn::{Item, ItemMod};

use crate::task::source_file::SourceFile;

pub struct BinaryFiles(BTreeSet<String>);

impl BinaryFiles {
    const PRODUCT_CRATES: &str = "crates/";
    const BINARY_ROOT: &str = "/src/main.rs";

    pub fn collect(files: &[SourceFile]) -> BinaryFiles {
        let by_path: BTreeMap<&str, &SourceFile> =
            files.iter().map(|file| (file.path(), file)).collect();
        let mut members = BTreeSet::new();
        files
            .iter()
            .filter(|file| {
                file.path().starts_with(Self::PRODUCT_CRATES)
                    && file.path().ends_with(Self::BINARY_ROOT)
            })
            .for_each(|root| Self::gather(root, &by_path, &mut members));
        BinaryFiles(members)
    }

    pub fn contains(&self, path: &str) -> bool {
        self.0.contains(path)
    }

    fn gather(
        file: &SourceFile,
        by_path: &BTreeMap<&str, &SourceFile>,
        members: &mut BTreeSet<String>,
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
            .filter_map(|candidate| by_path.get(candidate.as_str()).copied())
            .for_each(|child| Self::gather(child, by_path, members));
    }

    fn declared_files(parent: &str, module: &ItemMod) -> [String; 2] {
        let parent = Path::new(parent);
        let directory = if parent.ends_with("main.rs") || parent.ends_with("mod.rs") {
            parent.parent().map(Path::to_path_buf)
        } else {
            parent.with_extension("").into()
        }
        .unwrap_or_default();
        let name = module.ident.to_string();
        [
            directory.join(format!("{name}.rs")).display().to_string(),
            directory.join(&name).join("mod.rs").display().to_string(),
        ]
    }
}
