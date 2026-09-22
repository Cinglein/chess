use crate::lint::Lint;
use crate::wasm::Wasm;
use crate::workspace::Workspace;

pub struct Ci;

impl Ci {
    pub fn run(workspace: &Workspace) -> Result<(), String> {
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
