use std::collections::BTreeMap;

use quote::ToTokens;
use syn::visit::Visit;
use syn::{ImplItem, ItemImpl, Visibility};

use task::Report;
use task::Site;
use task::SourceFile;
use task::Violation;
use task::{Limit, Policy};

pub struct PrivateFns;

impl PrivateFns {
    #[must_use]
    pub fn report(files: &[SourceFile], policy: &Policy) -> Report {
        Report::new(
            "too many private fns; a type's logic belongs in its interface or in more types",
            files
                .iter()
                .flat_map(|file| {
                    Self::private_fns(file.syntax())
                        .into_iter()
                        .filter(|(_, names)| {
                            !policy.limit(Limit::PrivateFnsPerType).allows(names.len())
                        })
                        .map(move |(type_name, names)| {
                            Violation::new(
                                Site::File(file.path().into()),
                                format!(
                                    "{type_name} has {} private fns, at most {} allowed: {}",
                                    names.len(),
                                    policy.limit(Limit::PrivateFnsPerType).count(),
                                    names.join(", ")
                                ),
                            )
                        })
                })
                .collect(),
        )
    }

    fn private_fns(file: &syn::File) -> BTreeMap<String, Vec<String>> {
        let mut counter = PrivateFnCounter::default();
        counter.visit_file(file);
        counter.0
    }
}

#[derive(Default)]
struct PrivateFnCounter(BTreeMap<String, Vec<String>>);

impl<'ast> Visit<'ast> for PrivateFnCounter {
    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        if item.trait_.is_some() {
            return;
        }
        let type_name = item.self_ty.to_token_stream().to_string().replace(' ', "");
        let names = item.items.iter().filter_map(|item| match item {
            ImplItem::Fn(function) if matches!(function.vis, Visibility::Inherited) => {
                Some(function.sig.ident.to_string())
            }
            _ => None,
        });
        self.0.entry(type_name).or_default().extend(names);
    }
}

#[cfg(test)]
mod tests {
    use super::PrivateFns;
    use settings::EmbeddedDefaults;
    use task::{Policy, SourceFile};
    const SOURCE: &str = "
struct Wide;
impl Wide { fn a() {} fn b() {} pub fn c() {} }
impl Wide { fn d() {} fn e() {} fn g() {} pub(crate) fn f() {} }
impl Iterator for Wide { type Item = u8; fn next(&mut self) -> Option<u8> { None } }
struct Narrow;
impl Narrow { fn a() {} fn b() {} fn c() {} fn d() {} }
";
    const REPORTED: [&str; 1] = ["Wide has 5 private fns, at most 4 allowed: a, b, d, e, g"];

    #[test]
    fn counts_private_inherent_fns_across_impl_blocks_but_not_trait_impls() {
        let file = SourceFile::parse("wide.rs", SOURCE.to_owned()).unwrap();
        let report = PrivateFns::report(core::slice::from_ref(&file), &Policy::defaults());
        assert!(report.lists_exactly(&REPORTED), "{report}");
    }
}
