mod candidate;
mod method_shape;

pub use candidate::Candidate;

use std::collections::{BTreeMap, BTreeSet};

use method_shape::MethodShape;
use syn::visit::Visit;
use syn::{ImplItem, ItemImpl, ItemMod, ItemStruct, Type};

use crate::task::source_file::SourceFile;

#[derive(Default)]
pub struct BagCandidates {
    structs: BTreeMap<String, usize>,
    disqualified: BTreeSet<String>,
}

impl BagCandidates {
    pub fn in_file(file: &SourceFile) -> BagCandidates {
        let mut candidates = BagCandidates::default();
        candidates.visit_file(file.syntax());
        candidates
    }

    pub fn candidates(&self) -> impl Iterator<Item = Candidate> {
        self.structs
            .iter()
            .filter(|(name, _)| !self.disqualified.contains(*name))
            .map(|(name, line)| Candidate::new(name.clone(), *line))
    }

    fn self_name(item: &ItemImpl) -> Option<String> {
        let Type::Path(path) = &*item.self_ty else {
            return None;
        };
        path.path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
    }

    fn carries_logic(item: &ItemImpl, self_name: &str) -> bool {
        item.trait_.is_some()
            || item.items.iter().any(|member| match member {
                ImplItem::Fn(function) => !MethodShape::classify(function, self_name).is_trivial(),
                _ => false,
            })
    }
}

impl<'ast> Visit<'ast> for BagCandidates {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        self.structs
            .insert(item.ident.to_string(), item.ident.span().start().line);
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        if let Some(name) = Self::self_name(item).filter(|name| Self::carries_logic(item, name)) {
            self.disqualified.insert(name);
        }
    }
}
