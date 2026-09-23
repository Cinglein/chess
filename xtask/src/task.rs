use strum::{EnumString, VariantNames};

use crate::ci::Ci;
use crate::const_shape::ConstShape;
use crate::distinct_signatures::DistinctSignatures;
use crate::failure::Failure;
use crate::fn_shape::FnShape;
use crate::lint::Lint;
use crate::magics::Magics;
use crate::manual_iteration::ManualIteration;
use crate::named_lifetimes::NamedLifetimes;
use crate::no_comments::NoComments;
use crate::no_free_fns::NoFreeFns;
use crate::private_fns::PrivateFns;
use crate::state_graph::StateGraph;
use crate::test_budget::TestBudget;
use crate::type_shape::TypeShape;
use crate::wasm::Wasm;
use crate::workspace::Workspace;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum Task {
    Ci,
    ConstShape,
    DistinctSignatures,
    FnShape,
    Lint,
    Magics,
    ManualIteration,
    NamedLifetimes,
    NoComments,
    NoFreeFns,
    PrivateFns,
    StateGraph,
    TestBudget,
    TypeShape,
    Wasm,
}

impl Task {
    pub fn usage() -> Failure {
        Failure::Usage(format!("usage: cargo xtask <{}>", Self::VARIANTS.join("|")))
    }

    pub fn run(self) -> Result<(), Failure> {
        let workspace = Workspace::locate();
        match self {
            Task::Ci => Ci::run(&workspace),
            Task::ConstShape => ConstShape::check(&workspace.source_files()?).verdict(),
            Task::DistinctSignatures => {
                DistinctSignatures::check(&workspace.source_files()?).verdict()
            }
            Task::FnShape => FnShape::check(&workspace.source_files()?).verdict(),
            Task::Lint => Lint::run(&workspace),
            Task::Magics => Magics::run(&workspace),
            Task::ManualIteration => ManualIteration::check(&workspace.source_files()?).verdict(),
            Task::NamedLifetimes => NamedLifetimes::check(&workspace.source_files()?).verdict(),
            Task::NoComments => NoComments::check(&workspace.source_files()?).verdict(),
            Task::NoFreeFns => NoFreeFns::check(&workspace.source_files()?).verdict(),
            Task::PrivateFns => PrivateFns::check(&workspace.source_files()?).verdict(),
            Task::StateGraph => StateGraph::check(&workspace.source_files()?).verdict(),
            Task::TestBudget => TestBudget::check(&workspace.source_files()?).verdict(),
            Task::TypeShape => TypeShape::check(&workspace.source_files()?).verdict(),
            Task::Wasm => Wasm::run(&workspace),
        }
    }
}
