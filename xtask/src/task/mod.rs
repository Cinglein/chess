mod ci;
mod const_shape;
mod distinct_signatures;
mod failure;
mod fn_shape;
mod lint;
mod literal_names;
mod magics;
mod manual_iteration;
mod module_nesting;
mod named_lifetimes;
mod no_comments;
mod no_free_fns;
mod private_fns;
mod report;
mod site;
mod source_file;
mod state_graph;
mod test_budget;
mod type_shape;
mod violation;
mod wasm;
mod workspace;

use strum::{EnumString, VariantNames};

use crate::task::const_shape::ConstShape;
use crate::task::distinct_signatures::DistinctSignatures;
use crate::task::failure::Failure;
use crate::task::fn_shape::FnShape;
use crate::task::lint::Lint;
use crate::task::literal_names::LiteralNames;
use crate::task::manual_iteration::ManualIteration;
use crate::task::module_nesting::ModuleNesting;
use crate::task::named_lifetimes::NamedLifetimes;
use crate::task::no_comments::NoComments;
use crate::task::no_free_fns::NoFreeFns;
use crate::task::private_fns::PrivateFns;
use crate::task::state_graph::StateGraph;
use crate::task::test_budget::TestBudget;
use crate::task::type_shape::TypeShape;
use crate::task::wasm::Wasm;
use crate::task::workspace::Workspace;
use ci::Ci;
use magics::Magics;

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
    ModuleNesting,
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
            Task::ModuleNesting => ModuleNesting::report(&workspace.source_files()?).verdict(),
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
