use settings::EmbeddedDefaults;

use crate::task::clippy::Clippy;
use crate::task::failure::Failure;
use crate::task::lint::Lint;
use crate::task::policy::Policy;
use crate::task::selection::Selection;
use crate::task::test::Test;
use crate::task::test_time::TestTime;
use crate::task::wasm::Wasm;
use crate::task::workspace::Workspace;

pub struct Ci;

impl Ci {
    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        println!("ci: checking {selection}");
        workspace.cargo(&["fmt", "--all", "--check"])?;
        Clippy::run(workspace, selection)?;
        Wasm::run(workspace, selection)?;
        Test::run(workspace, selection)?;
        TestTime::run(workspace, selection, Policy::defaults().test_time())?;
        Lint::run(workspace)
    }
}
