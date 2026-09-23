use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Expr, ItemFn, ItemMod, LitInt, Macro, Token};

use super::TestBudget;
use super::measurement::Measurement;
use super::test_counts::TestCounts;
use crate::site::Site;
use crate::violation::Violation;

pub(super) struct TestModule<'scan> {
    path: &'scan str,
    budget: TestBudget,
}

impl<'scan> TestModule<'scan> {
    pub(super) fn budget(path: &'scan str, module: &ItemMod) -> TestBudget {
        let mut scan = TestModule {
            path,
            budget: TestBudget {
                test_lines: Self::line_count(module.span()),
                ..TestBudget::default()
            },
        };
        scan.visit_item_mod(module);
        scan.budget
    }

    fn test_violations(&self, function: &ItemFn) -> Vec<Violation> {
        let name = &function.sig.ident;
        let site = Site::Line(self.path.to_owned(), name.span().start().line);
        TestCounts::of(&function.block)
            .measurements()
            .into_iter()
            .filter(Measurement::exceeded)
            .map(|measurement| Violation::new(site.clone(), format!("fn {name} {measurement}")))
            .collect()
    }

    fn line_count(span: Span) -> usize {
        span.end().line - span.start().line + 1
    }

    fn is_test(function: &ItemFn) -> bool {
        function
            .attrs
            .iter()
            .any(|attribute| attribute.path().is_ident("test"))
    }
}

impl<'ast> Visit<'ast> for TestModule<'_> {
    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        if Self::is_test(function) {
            self.budget.tests += 1;
            self.budget
                .violations
                .extend(self.test_violations(function));
        }
        syn::visit::visit_item_fn(self, function);
    }

    fn visit_lit_int(&mut self, integer: &'ast LitInt) {
        if integer
            .base10_parse::<u64>()
            .is_ok_and(|value| value > TestBudget::MAX_INTEGER_LITERAL)
        {
            self.budget.violations.push(Violation::new(
                Site::Line(self.path.to_owned(), integer.span().start().line),
                format!(
                    "integer literal {integer} in test code, at most {} allowed",
                    TestBudget::MAX_INTEGER_LITERAL
                ),
            ));
        }
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
}
