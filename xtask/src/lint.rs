use crate::distinct_signatures::DistinctSignatures;
use crate::fn_shape::FnShape;
use crate::named_lifetimes::NamedLifetimes;
use crate::no_comments::NoComments;
use crate::no_free_fns::NoFreeFns;
use crate::private_fns::PrivateFns;
use crate::state_graph::StateGraph;
use crate::test_budget::TestBudget;
use crate::workspace::Workspace;

pub struct Lint;

impl Lint {
    pub fn run(workspace: &Workspace) -> Result<(), String> {
        let files = workspace.source_files()?;
        let failures: Vec<String> = [
            DistinctSignatures::check(&files),
            FnShape::check(&files),
            NamedLifetimes::check(&files),
            NoComments::check(&files),
            NoFreeFns::check(&files),
            PrivateFns::check(&files),
            StateGraph::check(&files),
            TestBudget::check(&files),
        ]
        .into_iter()
        .filter_map(Result::err)
        .collect();
        failures
            .is_empty()
            .then_some(())
            .ok_or_else(|| failures.join("\n\n"))
    }
}
