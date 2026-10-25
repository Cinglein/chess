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
        let (first, rest) = segments.split_first()?;
        let start = self.anchor(first)?;
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
                Item::Mod(module) => Some(module),
                _ => None,
            })
            .filter(|module| module.content.is_none())
            .map(|module| module.ident.to_string())
            .filter_map(|name| Self::deepest_known(&self.directory, &[name], known))
            .collect()
    }

    fn anchor(&self, first: &str) -> Option<PathBuf> {
        match first {
            "crate" => Some(PathBuf::from(Self::SOURCE)),
            "super" => Some(self.parent.clone()),
            "self" => Some(self.directory.clone()),
            _ => None,
        }
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    use super::ModuleFile;

    const KNOWN: [&str; 4] = [
        "xtask/src/t/mod.rs", "xtask/src/t/l.rs", "xtask/src/t/m/mod.rs", "xtask/src/t/m/s.rs",
    ];
    const USES: [(&str, &str, Option<&str>); 5] = [
        (
            "xtask/src/t/m/s.rs",
            "super::M",
            Some("xtask/src/t/m/mod.rs"),
        ),
        (
            "xtask/src/t/m/mod.rs",
            "self::s::S",
            Some("xtask/src/t/m/s.rs"),
        ),
        (
            "xtask/src/t/l.rs",
            "crate::t::m::s::S",
            Some("xtask/src/t/m/s.rs"),
        ),
        ("xtask/src/t/l.rs", "t::m::s::S", None),
        ("xtask/src/t/m/mod.rs", "s::S", None),
    ];

    #[test]
    fn use_paths_resolve_from_the_crate_root_the_parent_or_the_module_itself() {
        let known: BTreeSet<&Path> = KNOWN.iter().map(Path::new).collect();
        let resolved = USES.map(|(file, path, _)| {
            let segments: Vec<String> = path.split("::").map(str::to_owned).collect();
            ModuleFile::enclosing(Path::new(file)).resolve(&segments, &known)
        });
        assert_eq!(
            resolved,
            USES.map(|(_, _, target)| target.map(PathBuf::from))
        );
    }
}
