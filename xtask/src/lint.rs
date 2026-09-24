use crate::const_shape::ConstShape;
use crate::distinct_signatures::DistinctSignatures;
use crate::failure::Failure;
use crate::fn_shape::FnShape;
use crate::literal_names::LiteralNames;
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
            ConstShape::report(&files),
            DistinctSignatures::report(&files),
            FnShape::report(&files),
            LiteralNames::report(&files),
            ManualIteration::report(&files),
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
