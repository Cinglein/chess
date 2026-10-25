mod test_module;

use test_module::TestModule;

use std::path::Path;

use std::mem;

use syn::ItemMod;
use syn::visit::Visit;

use super::TestBudget;
use task::Site;
use task::SourceFile;
use task::Violation;
use task::Walk;
use task::{Limit, Policy};

pub(super) struct TestScan<'scan> {
    path: &'scan Path,
    policy: &'scan Policy,
    budget: TestBudget,
}

impl<'scan> TestScan<'scan> {
    pub(super) fn budget(path: &'scan Path, file: &syn::File, policy: &'scan Policy) -> TestBudget {
        let mut scan = TestScan {
            path,
            policy,
            budget: TestBudget::default(),
        };
        scan.visit_file(file);
        if !policy.limit(Limit::TestsPerFile).allows(scan.budget.tests) {
            scan.budget.violations.push(Violation::new(
                Site::File(path.into()),
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
        match SourceFile::walk(module) {
            Walk::Skipped => {
                self.budget = mem::take(&mut self.budget)
                    + TestModule::budget(self.path, module, self.policy);
            }
            Walk::Entered => syn::visit::visit_item_mod(self, module),
        }
    }
}
