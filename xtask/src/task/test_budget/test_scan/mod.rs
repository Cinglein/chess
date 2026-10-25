mod test_module;

use test_module::TestModule;

use std::mem;

use syn::ItemMod;
use syn::visit::Visit;

use super::TestBudget;
use crate::task::policy::{Limit, Policy};
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub(super) struct TestScan<'scan> {
    path: &'scan str,
    policy: &'scan Policy,
    budget: TestBudget,
}

impl<'scan> TestScan<'scan> {
    pub(super) fn budget(path: &'scan str, file: &syn::File, policy: &'scan Policy) -> TestBudget {
        let mut scan = TestScan {
            path,
            policy,
            budget: TestBudget::default(),
        };
        scan.visit_file(file);
        if !policy.limit(Limit::TestsPerFile).allows(scan.budget.tests) {
            scan.budget.violations.push(Violation::new(
                Site::File(path.to_owned()),
                format!(
                    "{} tests, at most {} allowed",
                    scan.budget.tests,
                    policy.limit(Limit::TestsPerFile).count()
                ),
            ));
        }
        scan.budget
    }
}

impl<'ast> Visit<'ast> for TestScan<'_> {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if SourceFile::is_test_module(module) {
            self.budget =
                mem::take(&mut self.budget) + TestModule::budget(self.path, module, self.policy);
        } else {
            syn::visit::visit_item_mod(self, module);
        }
    }
}
