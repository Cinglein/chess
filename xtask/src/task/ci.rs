use settings::EmbeddedDefaults;

use crate::task::affected::Job;
use crate::task::clippy::Clippy;
use crate::task::command_line::{CargoArgument, Program};
use crate::task::coverage::Coverage;
use crate::task::failure::Failure;
use crate::task::lint::Lint;
use crate::task::policy::Policy;
use crate::task::test::Test;
use crate::task::test_time::TestTime;
use crate::task::wasm::Wasm;
use crate::task::workspace::Workspace;

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
