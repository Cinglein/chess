use settings::EmbeddedDefaults;

use crate::task::const_shape::ConstShape;
use crate::task::distinct_signatures::DistinctSignatures;
use crate::task::expression_flow::ExpressionFlow;
use crate::task::failure::Failure;
use crate::task::fn_shape::FnShape;
use crate::task::literal_names::LiteralNames;
use crate::task::manual_iteration::ManualIteration;
use crate::task::module_nesting::ModuleNesting;
use crate::task::named_lifetimes::NamedLifetimes;
use crate::task::named_numbers::NamedNumbers;
use crate::task::no_comments::NoComments;
use crate::task::no_forwarders::NoForwarders;
use crate::task::no_free_fns::NoFreeFns;
use crate::task::no_numbers_in_binaries::NoNumbersInBinaries;
use crate::task::no_parameter_bags::NoParameterBags;
use crate::task::policy::Policy;
use crate::task::primitive_boundary::PrimitiveBoundary;
use crate::task::private_fns::PrivateFns;
use crate::task::public_surface::PublicSurface;
use crate::task::report::Report;
use crate::task::settings_shape::SettingsShape;
use crate::task::state_graph::StateGraph;
use crate::task::test_budget::TestBudget;
use crate::task::type_shape::TypeShape;
use crate::task::workspace::Workspace;

pub struct Lint;

impl Lint {
    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        let files = workspace.source_files()?;
        let policy = Policy::defaults();
        let reports: Vec<Report> = [
            ConstShape::report(&files),
            DistinctSignatures::report(&files),
            ExpressionFlow::report(&files),
            FnShape::report(&files, &policy),
            LiteralNames::report(&files, &policy),
            ManualIteration::report(&files),
            ModuleNesting::report(&files, &policy),
            NamedLifetimes::report(&files, &policy),
            NamedNumbers::report(&files),
            NoComments::report(&files),
            NoForwarders::report(&files),
            NoFreeFns::report(&files),
            NoNumbersInBinaries::report(&files),
            NoParameterBags::report(&files),
            PrimitiveBoundary::report(&files),
            PrivateFns::report(&files, &policy),
            PublicSurface::report(&files),
            SettingsShape::report(&files),
            StateGraph::report(&files, &policy),
            TestBudget::report(&files, &policy),
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
