use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Expr, ItemFn, ItemMod, LitInt, Macro, Token};

use super::TestBudget;
use super::test_counts::TestCounts;

pub(super) struct TestScan<'a> {
    path: &'a str,
    inside_test_module: bool,
    budget: TestBudget,
}

impl<'a> TestScan<'a> {
    pub(super) fn budget(path: &'a str, file: &syn::File) -> TestBudget {
        let mut scan = TestScan {
            path,
            inside_test_module: false,
            budget: TestBudget::default(),
        };
        scan.visit_file(file);
        scan.budget.violations.extend(scan.file_violation());
        scan.budget
    }

    fn file_violation(&self) -> Option<String> {
        (self.budget.tests > TestBudget::MAX_TESTS_PER_FILE).then(|| {
            format!(
                "{}: {} tests, at most {} allowed",
                self.path,
                self.budget.tests,
                TestBudget::MAX_TESTS_PER_FILE
            )
        })
    }

    fn test_violations(&self, function: &ItemFn) -> Vec<String> {
        let name = &function.sig.ident;
        let line = name.span().start().line;
        let counts = TestCounts::of(&function.block);
        [
            (
                "assertions",
                counts.assertions,
                TestBudget::MAX_ASSERTIONS_PER_TEST,
            ),
            (
                "body lines",
                Self::line_count(function.block.span()).saturating_sub(2),
                TestBudget::MAX_LINES_PER_TEST,
            ),
            (
                "literals",
                counts.literals,
                TestBudget::MAX_LITERALS_PER_TEST,
            ),
        ]
        .into_iter()
        .filter(|(_, actual, limit)| actual > limit)
        .map(|(kind, actual, limit)| {
            format!(
                "{}:{line}: fn {name} has {actual} {kind}, at most {limit} allowed",
                self.path
            )
        })
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

    fn is_cfg_test(module: &ItemMod) -> bool {
        module.attrs.iter().any(|attribute| {
            attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Ident>()
                    .is_ok_and(|ident| ident == "test")
        })
    }
}

impl<'ast> Visit<'ast> for TestScan<'_> {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        let outside = self.inside_test_module;
        if Self::is_cfg_test(module) {
            self.inside_test_module = true;
            self.budget.test_lines += Self::line_count(module.span());
        }
        syn::visit::visit_item_mod(self, module);
        self.inside_test_module = outside;
    }

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
        let too_large = integer
            .base10_parse::<u64>()
            .is_ok_and(|value| value > TestBudget::MAX_INTEGER_LITERAL);
        if self.inside_test_module && too_large {
            self.budget.violations.push(format!(
                "{}:{}: integer literal {integer} in test code, at most {} allowed",
                self.path,
                integer.span().start().line,
                TestBudget::MAX_INTEGER_LITERAL
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
