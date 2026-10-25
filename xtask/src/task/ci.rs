use settings::EmbeddedDefaults;

use crate::task::clippy::Clippy;
use crate::task::coverage::Coverage;
use crate::task::lint::Lint;
use crate::task::test::Test;
use crate::task::wasm::Wasm;
use affected::Job;
use task::Failure;
use task::Policy;
use task::Workspace;
use task::{CargoArgument, Program};
use test_time::TestTime;

pub struct Ci;

impl Ci {
    pub fn run(workspace: &Workspace, coverage: &Coverage) -> Result<(), Failure> {
        println!("ci: checking {}", coverage.selection(Job::Clippy));
        workspace
            .line(Program::Cargo)
            .argument(CargoArgument::Fmt)
            .argument(CargoArgument::All)
            .argument(CargoArgument::Check)
            .run()?;
        Clippy::run(workspace, &coverage.selection(Job::Clippy))?;
        Wasm::run(workspace, &coverage.selection(Job::Clippy))?;
        Test::run(workspace, &coverage.selection(Job::Test))?;
        TestTime::run(
            workspace,
            &coverage.selection(Job::Test),
            &Policy::defaults(),
        )?;
        Lint::run(workspace)
    }
}
