use syn::visit::Visit;
use syn::{Ident, ImplItemFn, ItemFn, ItemImpl, ItemMod, TraitItemFn};

use task::Policy;
use task::Report;
use task::Site;
use task::SourceFile;
use task::Violation;
use task::Walk;

pub struct LiteralNames;

impl LiteralNames {
    #[must_use]
    pub fn report(files: &[SourceFile], policy: &Policy) -> Report {
        Report::new(
            "function names say what the function does",
            files
                .iter()
                .flat_map(|file| {
                    Self::vague_names(file.syntax(), policy)
                        .into_iter()
                        .map(move |name| {
                            Violation::new(
                                Site::Line(file.path().into(), name.span().start().line),
                                format!("fn {name} names nothing; say what it does"),
                            )
                        })
                })
                .collect(),
        )
    }

    fn vague_names(file: &syn::File, policy: &Policy) -> Vec<Ident> {
        let mut names = VagueNames {
            policy,
            found: Vec::new(),
        };
        names.visit_file(file);
        names.found
    }
}

struct VagueNames<'scan> {
    policy: &'scan Policy,
    found: Vec<Ident>,
}

impl VagueNames<'_> {
    fn note(&mut self, name: &Ident) {
        if self.policy.is_vague(&name.to_string()) {
            self.found.push(name.clone());
        }
    }
}

impl<'ast> Visit<'ast> for VagueNames<'_> {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        match SourceFile::walk(module) {
            Walk::Entered => syn::visit::visit_item_mod(self, module),
            Walk::Skipped => {}
        }
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        match Walk::inherent_impls(item) {
            Walk::Entered => syn::visit::visit_item_impl(self, item),
            Walk::Skipped => {}
        }
    }

    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        self.note(&function.sig.ident);
        syn::visit::visit_item_fn(self, function);
    }

    fn visit_impl_item_fn(&mut self, function: &'ast ImplItemFn) {
        self.note(&function.sig.ident);
        syn::visit::visit_impl_item_fn(self, function);
    }

    fn visit_trait_item_fn(&mut self, function: &'ast TraitItemFn) {
        self.note(&function.sig.ident);
        syn::visit::visit_trait_item_fn(self, function);
    }
}

#[cfg(test)]
mod tests {
    use super::LiteralNames;
    use settings::EmbeddedDefaults;
    use task::Policy;
    const SOURCE: &str = "
struct W;
impl W { fn of(x: u8) -> W { W } fn check(&self) {} fn new() -> W { W } fn least_significant_bit(&self) -> u8 { 0 } }
impl From<u8> for W { fn from(x: u8) -> W { W } } trait T { fn do_it(&self); fn go(&self); }
fn process() {} mod inner { fn handle() {} }
#[cfg(test)] mod tests { fn make() {} }
";
    const FLAGGED: [&str; 5] = ["of", "check", "go", "process", "handle"];

    #[test]
    fn flags_prepositions_and_vague_verbs_but_not_new_trait_impls_or_tests() {
        let parsed = syn::parse_file(SOURCE).unwrap();
        let names: Vec<String> = LiteralNames::vague_names(&parsed, &Policy::defaults())
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(names, FLAGGED);
    }
}
