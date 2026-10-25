use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use syn::Item;

pub struct ModuleFile {
    directory: PathBuf,
    parent: PathBuf,
}

impl ModuleFile {
    const ROOTS: &[&str] = &["mod.rs", "lib.rs", "main.rs"];
    const SOURCE: &str = "xtask/src";

    pub fn enclosing(path: &Path) -> ModuleFile {
        let above = path.parent().map(Path::to_path_buf).unwrap_or_default();
        if path
            .file_name()
            .is_some_and(|name| Self::ROOTS.iter().any(|root| name == *root))
        {
            ModuleFile {
                parent: above.parent().map(Path::to_path_buf).unwrap_or_default(),
                directory: above,
            }
        } else {
            ModuleFile {
                directory: path.with_extension(""),
                parent: above,
            }
        }
    }

    pub fn resolve(&self, segments: &[String], known: &BTreeSet<&Path>) -> Option<PathBuf> {
        let (start, rest) = match segments.split_first()? {
            (first, rest) if first == "crate" => (PathBuf::from(Self::SOURCE), rest),
            (first, rest) if first == "super" => (self.parent.clone(), rest),
            (first, rest) if first == "self" => (self.directory.clone(), rest),
            _ => return None,
        };
        let supers = rest
            .iter()
            .take_while(|segment| *segment == "super")
            .count();
        let start = (0..supers).fold(start, |directory, _| {
            directory
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_default()
        });
        Self::deepest_known(&start, &rest[supers..], known)
    }

    pub fn declared_children(&self, syntax: &syn::File, known: &BTreeSet<&Path>) -> Vec<PathBuf> {
        syntax
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(module) if module.content.is_none() => Some(module.ident.to_string()),
                _ => None,
            })
            .filter_map(|name| Self::deepest_known(&self.directory, &[name], known))
            .collect()
    }

    fn deepest_known(start: &Path, rest: &[String], known: &BTreeSet<&Path>) -> Option<PathBuf> {
        (0..=rest.len()).rev().find_map(|depth| {
            let directory = rest[..depth]
                .iter()
                .fold(start.to_path_buf(), |path, segment| path.join(segment));
            [directory.with_extension("rs"), directory.join("mod.rs")]
                .into_iter()
                .find(|candidate| known.contains(candidate.as_path()))
        })
    }
}
