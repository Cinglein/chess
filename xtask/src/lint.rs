use crate::const_shape::ConstShape;
use crate::distinct_signatures::DistinctSignatures;
use crate::failure::Failure;
use crate::fn_shape::FnShape;
use crate::manual_iteration::ManualIteration;
use crate::named_lifetimes::NamedLifetimes;
use crate::no_comments::NoComments;
use crate::no_free_fns::NoFreeFns;
use crate::private_fns::PrivateFns;
use crate::report::Report;
use crate::state_graph::StateGraph;
use crate::test_budget::TestBudget;
use crate::type_shape::TypeShape;
use crate::workspace::Workspace;

pub struct Lint;

impl Lint {
    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        let files = workspace.source_files()?;
        let reports: Vec<Report> = [
            ConstShape::check(&files),
            DistinctSignatures::check(&files),
            FnShape::check(&files),
            ManualIteration::check(&files),
            NamedLifetimes::check(&files),
            NoComments::check(&files),
            NoFreeFns::check(&files),
            PrivateFns::check(&files),
            StateGraph::check(&files),
            TestBudget::check(&files),
            TypeShape::check(&files),
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
