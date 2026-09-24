use crate::task::const_shape::ConstShape;
use crate::task::distinct_signatures::DistinctSignatures;
use crate::task::failure::Failure;
use crate::task::fn_shape::FnShape;
use crate::task::literal_names::LiteralNames;
use crate::task::manual_iteration::ManualIteration;
use crate::task::module_nesting::ModuleNesting;
use crate::task::named_lifetimes::NamedLifetimes;
use crate::task::no_comments::NoComments;
use crate::task::no_free_fns::NoFreeFns;
use crate::task::private_fns::PrivateFns;
use crate::task::report::Report;
use crate::task::state_graph::StateGraph;
use crate::task::test_budget::TestBudget;
use crate::task::type_shape::TypeShape;
use crate::task::workspace::Workspace;

pub struct Lint;

impl Lint {
    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        let files = workspace.source_files()?;
        let reports: Vec<Report> = [
            ConstShape::report(&files),
            DistinctSignatures::report(&files),
            FnShape::report(&files),
            LiteralNames::report(&files),
            ManualIteration::report(&files),
            ModuleNesting::report(&files),
            NamedLifetimes::report(&files),
            NoComments::report(&files),
            NoFreeFns::report(&files),
            PrivateFns::report(&files),
            StateGraph::report(&files),
            TestBudget::report(&files),
            TypeShape::report(&files),
        ]
        .into_iter()
        .filter(|report| !report.is_clean())
        .collect();
        if reports.is_empty() {
            println!("lint: {} rust files clean", files.len());
            Ok(())
        } else {
            Err(Failure::Lint(reports))
        }
    }
}
