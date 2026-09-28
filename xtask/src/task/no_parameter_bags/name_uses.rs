use std::collections::BTreeSet;

use syn::Ident;
use syn::visit::Visit;

use crate::task::source_file::SourceFile;

#[derive(Default)]
pub struct NameUses(BTreeSet<String>);

impl NameUses {
    pub fn in_file(file: &SourceFile) -> NameUses {
        let mut uses = NameUses::default();
        uses.visit_file(file.syntax());
        uses
    }

    pub fn mentions(&self, name: &str) -> bool {
        self.0.contains(name)
    }
}

impl<'ast> Visit<'ast> for NameUses {
    fn visit_ident(&mut self, ident: &'ast Ident) {
        self.0.insert(ident.to_string());
    }
}
