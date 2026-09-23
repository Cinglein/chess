use strum::{EnumString, VariantNames};

use crate::ci::Ci;
use crate::const_shape::ConstShape;
use crate::distinct_signatures::DistinctSignatures;
use crate::failure::Failure;
use crate::fn_shape::FnShape;
use crate::lint::Lint;
use crate::literal_names::LiteralNames;
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
    LiteralNames,
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
            Task::ConstShape => ConstShape::report(&workspace.source_files()?).verdict(),
            Task::DistinctSignatures => {
                DistinctSignatures::report(&workspace.source_files()?).verdict()
            }
            Task::FnShape => FnShape::report(&workspace.source_files()?).verdict(),
            Task::Lint => Lint::run(&workspace),
            Task::LiteralNames => LiteralNames::report(&workspace.source_files()?).verdict(),
            Task::Magics => Magics::run(&workspace),
            Task::ManualIteration => ManualIteration::report(&workspace.source_files()?).verdict(),
            Task::NamedLifetimes => NamedLifetimes::report(&workspace.source_files()?).verdict(),
            Task::NoComments => NoComments::report(&workspace.source_files()?).verdict(),
            Task::NoFreeFns => NoFreeFns::report(&workspace.source_files()?).verdict(),
            Task::PrivateFns => PrivateFns::report(&workspace.source_files()?).verdict(),
            Task::StateGraph => StateGraph::report(&workspace.source_files()?).verdict(),
            Task::TestBudget => TestBudget::report(&workspace.source_files()?).verdict(),
            Task::TypeShape => TypeShape::report(&workspace.source_files()?).verdict(),
            Task::Wasm => Wasm::run(&workspace),
        }
    }
}
