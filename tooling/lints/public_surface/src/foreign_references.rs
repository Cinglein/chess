use std::collections::BTreeSet;

use syn::visit::Visit;
use syn::{ItemUse, Path, UseTree};

use super::export::Export;
use task::BinaryFiles;
use task::CrateName;
use task::SourceFile;

#[derive(Default)]
pub struct ForeignReferences {
    own_crate: Option<CrateName>,
    names: BTreeSet<String>,
    crates: BTreeSet<CrateName>,
}

impl ForeignReferences {
    pub fn in_file(file: &SourceFile, binaries: &BinaryFiles) -> ForeignReferences {
        let own_crate = file
            .crate_name()
            .filter(|_| !binaries.contains(file.path()))
            .cloned();
        let mut references = ForeignReferences {
            own_crate,
            names: BTreeSet::new(),
            crates: BTreeSet::new(),
        };
        references.visit_file(file.syntax());
        references
    }

    pub fn absorb(mut self, other: ForeignReferences) -> ForeignReferences {
        self.names.extend(other.names);
        self.crates.extend(other.crates);
        self
    }

    pub fn depends_on(&self, crate_name: &CrateName) -> bool {
        self.crates.contains(crate_name)
    }

    pub fn names(&self, export: &Export) -> bool {
        self.names
            .contains(&Self::key(export.crate_name(), export.name()))
    }

    fn key(crate_name: &CrateName, name: &str) -> String {
        format!("{crate_name}::{name}")
    }

    fn walk(&mut self, first: Option<&str>, tree: &UseTree) {
        match tree {
            UseTree::Path(path) => {
                let ident = path.ident.to_string();
                self.walk(Some(first.unwrap_or(&ident)), &path.tree);
            }
            UseTree::Group(group) => group.items.iter().for_each(|item| self.walk(first, item)),
            UseTree::Name(name) => self.record(first, &name.ident.to_string()),
            UseTree::Rename(rename) => self.record(first, &rename.ident.to_string()),
            UseTree::Glob(_) => {}
        }
    }

    fn record(&mut self, first: Option<&str>, name: &str) {
        first
            .and_then(|crate_name| crate_name.parse::<CrateName>().ok())
            .filter(|crate_name| Some(crate_name) != self.own_crate.as_ref())
            .into_iter()
            .for_each(|crate_name| {
                self.names.insert(Self::key(&crate_name, name));
                self.crates.insert(crate_name);
            });
    }
}

impl<'ast> Visit<'ast> for ForeignReferences {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        self.walk(None, &item.tree);
    }

    fn visit_path(&mut self, path: &'ast Path) {
        path.segments
            .first()
            .zip(path.segments.iter().nth(1))
            .into_iter()
            .for_each(|(first, second)| {
                self.record(Some(&first.ident.to_string()), &second.ident.to_string());
            });
        syn::visit::visit_path(self, path);
    }
}
