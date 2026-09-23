use std::collections::BTreeMap;

use quote::ToTokens;
use syn::visit::Visit;
use syn::{ImplItem, ItemImpl, Visibility};

use crate::report::Report;
use crate::site::Site;
use crate::source_file::SourceFile;
use crate::violation::Violation;

pub struct PrivateFns;

impl PrivateFns {
    const MAX_PRIVATE_FNS_PER_TYPE: usize = 4;

    pub fn check(files: &[SourceFile]) -> Report {
        Report::new(
            "too many private fns; a type's logic belongs in its interface or in more types",
            files
                .iter()
                .flat_map(|file| {
                    Self::private_fns(file.syntax())
                        .into_iter()
                        .filter(|(_, names)| names.len() > Self::MAX_PRIVATE_FNS_PER_TYPE)
                        .map(move |(type_name, names)| {
                            Violation::new(
                                Site::File(file.path().to_owned()),
                                format!(
                                    "{type_name} has {} private fns, at most {} allowed: {}",
                                    names.len(),
                                    Self::MAX_PRIVATE_FNS_PER_TYPE,
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

    const SOURCE: &str = "
struct Wide;
impl Wide { fn a() {} fn b() {} pub fn c() {} }
impl Wide { fn d() {} fn e() {} pub(crate) fn f() {} }
impl Iterator for Wide { type Item = u8; fn next(&mut self) -> Option<u8> { None } }
";
    const HIDDEN: [&str; 4] = ["a", "b", "d", "e"];

    #[test]
    fn counts_private_inherent_fns_across_impl_blocks_but_not_trait_impls() {
        let parsed = syn::parse_file(SOURCE).expect("valid rust");
        let counted = PrivateFns::private_fns(&parsed);
        assert_eq!(
            counted.get("Wide").map(Vec::as_slice),
            Some(HIDDEN.map(String::from).as_slice())
        );
    }
}
