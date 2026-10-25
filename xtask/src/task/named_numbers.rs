use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::{
    Expr, ExprConst, ImplItemConst, ItemConst, ItemImpl, ItemMod, LitFloat, LitInt, Macro, Token,
    TraitItemConst,
};

use crate::task::bit_boundary::BitBoundary;
use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::support_modules::SupportModules;
use crate::task::violation::Violation;

pub struct NamedNumbers;

impl NamedNumbers {
    const COVERED: &str = "crates/";
    const UNITS: [&str; 4] = ["0", "1", "0.0", "1.0"];

    pub fn report(files: &[SourceFile]) -> Report {
        let support: Vec<String> = files.iter().flat_map(SupportModules::paths).collect();
        Report::new(
            "a number has a name: outside tests, a literal other than 0 and 1 sits in a const item",
            files
                .iter()
                .filter(|file| {
                    file.path().starts_with(Self::COVERED)
                        && !BitBoundary::covers(file.path())
                        && !support.contains(&file.path().to_owned())
                })
                .flat_map(|file| {
                    let mut numbers = Numbers::default();
                    numbers.visit_file(file.syntax());
                    numbers.lines.into_iter().map(move |line| {
                        Violation::new(
                            Site::Line(file.path().to_owned(), line),
                            "numeric literal outside a const item; name it as a const so it can be found and tuned",
                        )
                    })
                })
                .collect(),
        )
    }

    fn is_unit(digits: &str) -> bool {
        Self::UNITS.contains(&digits.trim_start_matches('-'))
    }

    fn is_test_only(item: &ItemImpl) -> bool {
        item.attrs.iter().any(|attribute| {
            attribute.path().is_ident("cfg")
                && attribute
                    .meta
                    .to_token_stream()
                    .to_string()
                    .contains("test")
        })
    }
}

#[derive(Default)]
struct Numbers {
    named: usize,
    lines: Vec<usize>,
}

impl<'ast> Visit<'ast> for Numbers {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        if !NamedNumbers::is_test_only(item) {
            syn::visit::visit_item_impl(self, item);
        }
    }

    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        self.named += 1;
        syn::visit::visit_item_const(self, item);
        self.named -= 1;
    }

    fn visit_impl_item_const(&mut self, item: &'ast ImplItemConst) {
        self.named += 1;
        syn::visit::visit_impl_item_const(self, item);
        self.named -= 1;
    }

    fn visit_trait_item_const(&mut self, item: &'ast TraitItemConst) {
        self.named += 1;
        syn::visit::visit_trait_item_const(self, item);
        self.named -= 1;
    }

    fn visit_expr_const(&mut self, block: &'ast ExprConst) {
        self.named += 1;
        syn::visit::visit_expr_const(self, block);
        self.named -= 1;
    }

    fn visit_macro(&mut self, invocation: &'ast Macro) {
        if let Ok(arguments) =
            invocation.parse_body_with(Punctuated::<Expr, Token![,]>::parse_terminated)
        {
            arguments
                .iter()
                .for_each(|argument| self.visit_expr(argument));
        }
    }

    fn visit_lit_int(&mut self, literal: &'ast LitInt) {
        if self.named == 0 && !NamedNumbers::is_unit(literal.base10_digits()) {
            self.lines.push(literal.span().start().line);
        }
    }

    fn visit_lit_float(&mut self, literal: &'ast LitFloat) {
        if self.named == 0 && !NamedNumbers::is_unit(literal.base10_digits()) {
            self.lines.push(literal.span().start().line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NamedNumbers;
    use crate::task::source_file::SourceFile;

    const SOURCE: &str = "
        pub struct Clock;
        impl Clock {
            const HALF: u64 = 500;
            pub fn halved(ticks: u64) -> u64 { ticks / 2 + 1 }
            pub fn scaled(ticks: f64) -> f64 { ticks * 0.5 }
            pub fn named(ticks: u64) -> u64 { ticks / Self::HALF }
        }
        #[cfg(any(test, feature = \"proptest\"))]
        impl Clock { pub fn range() -> u64 { 60 } }
    ";
    const REPORTED: [&str; 2] = [":5:", ":6:"];
    const CLEAN: [&str; 3] = [":4:", ":7:", ":10:"];

    #[test]
    fn flags_literals_outside_const_items_but_not_units_consts_or_test_only_impls() {
        let file = SourceFile::parse("crates/a/src/clock.rs".to_owned(), SOURCE.to_owned())
            .expect("valid rust");
        let report = NamedNumbers::report(&[file]).to_string();
        assert!(
            REPORTED.iter().all(|line| report.contains(line))
                && CLEAN.iter().all(|line| !report.contains(line)),
            "{report}"
        );
    }
}
