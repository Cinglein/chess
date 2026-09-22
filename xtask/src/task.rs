use strum::{EnumString, VariantNames};

use crate::ci::Ci;
use crate::distinct_signatures::DistinctSignatures;
use crate::fn_shape::FnShape;
use crate::lint::Lint;
use crate::magics::Magics;
use crate::named_lifetimes::NamedLifetimes;
use crate::no_comments::NoComments;
use crate::no_free_fns::NoFreeFns;
use crate::private_fns::PrivateFns;
use crate::state_graph::StateGraph;
use crate::test_budget::TestBudget;
use crate::wasm::Wasm;
use crate::workspace::Workspace;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum Task {
    Ci,
    DistinctSignatures,
    FnShape,
    Lint,
    Magics,
    NamedLifetimes,
    NoComments,
    NoFreeFns,
    PrivateFns,
    StateGraph,
    TestBudget,
    Wasm,
}

impl Task {
    pub fn usage() -> String {
        format!("usage: cargo xtask <{}>", Self::VARIANTS.join("|"))
    }

    pub fn run(self) -> Result<(), String> {
        let workspace = Workspace::locate();
        match self {
            Task::Ci => Ci::run(&workspace),
            Task::DistinctSignatures => DistinctSignatures::check(&workspace.source_files()?),
            Task::FnShape => FnShape::check(&workspace.source_files()?),
            Task::Lint => Lint::run(&workspace),
            Task::Magics => Magics::run(&workspace),
            Task::NamedLifetimes => NamedLifetimes::check(&workspace.source_files()?),
            Task::NoComments => NoComments::check(&workspace.source_files()?),
            Task::NoFreeFns => NoFreeFns::check(&workspace.source_files()?),
            Task::PrivateFns => PrivateFns::check(&workspace.source_files()?),
            Task::StateGraph => StateGraph::check(&workspace.source_files()?),
            Task::TestBudget => TestBudget::check(&workspace.source_files()?),
            Task::Wasm => Wasm::run(&workspace),
        }
    }
}
