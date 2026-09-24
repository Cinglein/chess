use syn::visit::Visit;
use syn::{Ident, ImplItemFn, ItemFn, ItemImpl, ItemMod, TraitItemFn};

use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub struct LiteralNames;

impl LiteralNames {
    const MIN_LENGTH: usize = 3;
    const VAGUE: [&str; 24] = [
        "with", "from", "into", "for", "get", "set", "make", "check", "build", "create", "init",
        "process", "handle", "helper", "util", "data", "value", "item", "thing", "execute",
        "perform", "update", "compute", "manage",
    ];

    pub fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "function names say what the function does",
            files
                .iter()
                .flat_map(|file| {
                    Self::vague_names(file.syntax())
                        .into_iter()
                        .map(move |name| {
                            Violation::new(
                                Site::Line(file.path().to_owned(), name.span().start().line),
                                format!("fn {name} names nothing; say what it does"),
                            )
                        })
                })
                .collect(),
        )
    }

    fn vague_names(file: &syn::File) -> Vec<Ident> {
        let mut names = VagueNames::default();
        names.visit_file(file);
        names.0
    }

    fn is_vague(name: &Ident) -> bool {
        let text = name.to_string();
        text.len() < Self::MIN_LENGTH || Self::VAGUE.contains(&text.as_str())
    }
}

#[derive(Default)]
struct VagueNames(Vec<Ident>);

impl<'ast> Visit<'ast> for VagueNames {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        if item.trait_.is_none() {
            syn::visit::visit_item_impl(self, item);
        }
    }

    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        if LiteralNames::is_vague(&function.sig.ident) {
            self.0.push(function.sig.ident.clone());
        }
        syn::visit::visit_item_fn(self, function);
    }

    fn visit_impl_item_fn(&mut self, function: &'ast ImplItemFn) {
        if LiteralNames::is_vague(&function.sig.ident) {
            self.0.push(function.sig.ident.clone());
        }
        syn::visit::visit_impl_item_fn(self, function);
    }

    fn visit_trait_item_fn(&mut self, function: &'ast TraitItemFn) {
        if LiteralNames::is_vague(&function.sig.ident) {
            self.0.push(function.sig.ident.clone());
        }
        syn::visit::visit_trait_item_fn(self, function);
    }
}

#[cfg(test)]
mod tests {
    use super::LiteralNames;

    const SOURCE: &str = "
struct W;
impl W {
    fn of(x: u8) -> W { W }
    fn check(&self) {}
    fn new() -> W { W }
    fn least_significant_bit(&self) -> u8 { 0 }
}
impl From<u8> for W { fn from(x: u8) -> W { W } }
trait T { fn do_it(&self); fn go(&self); }
";
    const FLAGGED: [&str; 3] = ["of", "check", "go"];

    #[test]
    fn flags_prepositions_and_vague_verbs_but_not_new_or_trait_impls() {
        let parsed = syn::parse_file(SOURCE).expect("valid rust");
        let names: Vec<String> = LiteralNames::vague_names(&parsed)
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(names, FLAGGED);
    }
}
