mod macro_arguments;
mod scope;
mod test_counts;

use macro_arguments::MacroArguments;
use scope::Scope;
use test_counts::TestCounts;

use proc_macro2::Span;
use std::path::Path;
use syn::spanned::Spanned;
use syn::visit::Visit;

use syn::{ItemFn, ItemMod, LitInt, Macro};

use super::super::TestBudget;
use super::super::measurement::Measurement;
use crate::task::policy::{Limit, Policy};
use crate::task::site::Site;
use crate::task::violation::Violation;

pub(super) struct TestModule<'scan> {
    path: &'scan Path,
    policy: &'scan Policy,
    scope: Scope,
    budget: TestBudget,
}

impl<'scan> TestModule<'scan> {
    pub(super) fn budget(path: &'scan Path, module: &ItemMod, policy: &'scan Policy) -> TestBudget {
        let mut scan = TestModule {
            path,
            policy,
            scope: Scope::OutsideTest,
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
        let site = Site::Line(self.path.into(), name.span().start().line);
        TestCounts::tally(&function.block)
            .measurements(self.policy)
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
            self.scope = Scope::InsideTest;
        }
        syn::visit::visit_item_fn(self, function);
        self.scope = Scope::OutsideTest;
    }

    fn visit_lit_int(&mut self, integer: &'ast LitInt) {
        if integer
            .base10_parse::<usize>()
            .is_ok_and(|value| value > self.policy.limit(Limit::LargestInteger).count())
        {
            self.budget.violations.push(Violation::new(
                Site::Line(self.path.into(), integer.span().start().line),
                format!(
                    "integer literal {integer} in test code, at most {} allowed",
                    self.policy.limit(Limit::LargestInteger).count()
                ),
            ));
        }
    }

    fn visit_macro(&mut self, invocation: &'ast Macro) {
        if self.scope == Scope::OutsideTest && TestCounts::is_assertion(invocation) {
            self.budget.violations.push(Violation::new(
                Site::Line(self.path.into(), invocation.span().start().line),
                "assertion outside a test; helpers return values and tests assert them",
            ));
        }
        MacroArguments::parse(invocation).visit_with(self);
    }
}
