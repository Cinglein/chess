use crate::workspace::Workspace;

pub struct Wasm;

impl Wasm {
    const CRATES: &[&str] = &["board", "eval", "fen", "search"];

    pub fn run(workspace: &Workspace) -> Result<(), String> {
        Self::CRATES.iter().try_for_each(|crate_name| {
            workspace.cargo(&[
                "clippy",
                "--package",
                crate_name,
                "--target",
                "wasm32-unknown-unknown",
                "--",
                "-D",
                "warnings",
            ])
        })
    }
}
