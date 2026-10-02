use std::collections::BTreeSet;

use syn::visit::Visit;
use syn::{ItemUse, Path, UseTree};

use super::export::Export;
use crate::task::binary_files::BinaryFiles;
use crate::task::source_file::SourceFile;

#[derive(Default)]
pub struct ForeignReferences {
    own_crate: String,
    names: BTreeSet<String>,
    crates: BTreeSet<String>,
}

impl ForeignReferences {
    pub fn in_file(file: &SourceFile, binaries: &BinaryFiles) -> ForeignReferences {
        let own_crate = file
            .crate_name()
            .filter(|_| !binaries.contains(file.path()))
            .unwrap_or_default()
            .to_owned();
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

    pub fn depends_on(&self, crate_name: &str) -> bool {
        self.crates.contains(crate_name)
    }

    pub fn names(&self, export: &Export) -> bool {
        self.names
            .contains(&Self::key(export.crate_name(), export.name()))
    }

    fn key(crate_name: &str, name: &str) -> String {
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
        if let Some(crate_name) = first.filter(|crate_name| *crate_name != self.own_crate) {
            self.names.insert(Self::key(crate_name, name));
            self.crates.insert(crate_name.to_owned());
        }
    }
}

impl<'ast> Visit<'ast> for ForeignReferences {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        self.walk(None, &item.tree);
    }

    fn visit_path(&mut self, path: &'ast Path) {
        if let [first, second, ..] = path.segments.iter().collect::<Vec<_>>().as_slice() {
            self.record(Some(&first.ident.to_string()), &second.ident.to_string());
        }
        syn::visit::visit_path(self, path);
    }
}
