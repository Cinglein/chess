use crate::task::failure::Failure;
use crate::task::lint::Lint;
use crate::task::test_time::TestTime;
use crate::task::wasm::Wasm;
use crate::task::workspace::Workspace;

pub struct Ci;

impl Ci {
    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        workspace.cargo(&["fmt", "--all", "--check"])?;
        workspace.cargo(&[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ])?;
        Wasm::run(workspace)?;
        workspace.cargo(&["test", "--workspace", "--all-features"])?;
        TestTime::run(workspace)?;
        Lint::run(workspace)
    }
}
