mod ci;
mod clippy;
mod coverage;
mod lint;
mod test;
mod wasm;

use settings::EmbeddedDefaults;
use strum::{EnumString, VariantNames};

use crate::task::clippy::Clippy;
use crate::task::coverage::Coverage;
use crate::task::lint::Lint;
use crate::task::test::Test;
use crate::task::wasm::Wasm;
use affected::Affected;
use affected::Job;
use ci::Ci;
use const_shape::ConstShape;
use distinct_signatures::DistinctSignatures;
use expression_flow::ExpressionFlow;
use fn_shape::FnShape;
use literal_names::LiteralNames;
use magics::Magics;
use manual_iteration::ManualIteration;
use module_nesting::ModuleNesting;
use mutation::{Mutants, Scope};
use named_lifetimes::NamedLifetimes;
use named_numbers::NamedNumbers;
use no_comments::NoComments;
use no_forwarders::NoForwarders;
use no_free_fns::NoFreeFns;
use no_numbers_in_binaries::NoNumbersInBinaries;
use no_parameter_bags::NoParameterBags;
use primitive_boundary::PrimitiveBoundary;
use private_fns::PrivateFns;
use public_surface::PublicSurface;
use settings_shape::SettingsShape;
use stacked::Stacked;
use state_graph::StateGraph;
use task::Failure;
use task::Policy;
use task::Workspace;
use test_budget::TestBudget;
use test_time::TestTime;
use type_shape::TypeShape;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum Task {
    Affected,
    Ci,
    Clippy,
    ConstShape,
    DistinctSignatures,
    ExpressionFlow,
    FnShape,
    Lint,
    LiteralNames,
    Magics,
    ManualIteration,
    ModuleNesting,
    Mutants,
    MutantsFull,
    NamedLifetimes,
    NamedNumbers,
    NoComments,
    NoForwarders,
    NoFreeFns,
    NoNumbersInBinaries,
    NoParameterBags,
    PrimitiveBoundary,
    PrivateFns,
    PublicSurface,
    SettingsShape,
    Stacked,
    StateGraph,
    Test,
    TestBudget,
    TestTime,
    TypeShape,
    Wasm,
}

impl Task {
    #[must_use]
    pub fn usage() -> Failure {
        Failure::Usage(format!(
            "usage: cargo xtask <{}> [--affected]",
            Self::VARIANTS.join("|")
        ))
    }

    pub fn run(self, flags: impl Iterator<Item = String>) -> Result<(), Failure> {
        let workspace = Workspace::locate();
        let policy = Policy::defaults();
        let coverage = Coverage::from_flags(flags, &workspace)?;
        match self {
            Task::Affected => Affected::run(&workspace, &policy, coverage.plan(&workspace)?),
            Task::Ci => Ci::run(&workspace, &coverage),
            Task::Clippy => Clippy::run(&workspace, &coverage.selection(Job::Clippy)),
            Task::ConstShape => ConstShape::report(&workspace.source_files()?).verdict(),
            Task::DistinctSignatures => {
                DistinctSignatures::report(&workspace.source_files()?).verdict()
            }
            Task::ExpressionFlow => ExpressionFlow::report(&workspace.source_files()?).verdict(),
            Task::FnShape => FnShape::report(&workspace.source_files()?, &policy).verdict(),
            Task::Lint => Lint::run(&workspace),
            Task::LiteralNames => {
                LiteralNames::report(&workspace.source_files()?, &policy).verdict()
            }
            Task::Magics => Magics::run(&workspace),
            Task::ManualIteration => ManualIteration::report(&workspace.source_files()?).verdict(),
            Task::ModuleNesting => {
                ModuleNesting::report(&workspace.source_files()?, &policy).verdict()
            }
            Task::Mutants => Mutants::run(
                &workspace,
                Scope::ChangedLines,
                &coverage.selection(Job::Mutants),
                &policy,
            ),
            Task::MutantsFull => Mutants::run(
                &workspace,
                Scope::Workspace,
                &coverage.selection(Job::Full),
                &policy,
            ),
            Task::NamedLifetimes => {
                NamedLifetimes::report(&workspace.source_files()?, &policy).verdict()
            }
            Task::NamedNumbers => NamedNumbers::report(&workspace.source_files()?).verdict(),
            Task::NoComments => NoComments::report(&workspace.source_files()?).verdict(),
            Task::NoForwarders => NoForwarders::report(&workspace.source_files()?).verdict(),
            Task::NoFreeFns => NoFreeFns::report(&workspace.source_files()?).verdict(),
            Task::NoNumbersInBinaries => {
                NoNumbersInBinaries::report(&workspace.source_files()?).verdict()
            }
            Task::NoParameterBags => NoParameterBags::report(&workspace.source_files()?).verdict(),
            Task::PrimitiveBoundary => {
                PrimitiveBoundary::report(&workspace.source_files()?).verdict()
            }
            Task::PrivateFns => PrivateFns::report(&workspace.source_files()?, &policy).verdict(),
            Task::PublicSurface => PublicSurface::report(&workspace.source_files()?).verdict(),
            Task::SettingsShape => SettingsShape::report(&workspace.source_files()?).verdict(),
            Task::Stacked => Stacked::run(),
            Task::StateGraph => StateGraph::report(&workspace.source_files()?, &policy).verdict(),
            Task::Test => Test::run(&workspace, &coverage.selection(Job::Test)),
            Task::TestBudget => TestBudget::report(&workspace.source_files()?, &policy).verdict(),
            Task::TestTime => TestTime::run(&workspace, &coverage.selection(Job::Test), &policy),
            Task::TypeShape => TypeShape::report(&workspace.source_files()?).verdict(),
            Task::Wasm => Wasm::run(&workspace, &coverage.selection(Job::Clippy)),
        }
    }
}
