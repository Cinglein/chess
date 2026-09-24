use crate::task::failure::Failure;
use crate::task::lint::Lint;
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
        Lint::run(workspace)
    }
}
