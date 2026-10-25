use crate::task::failure::Failure;
use crate::task::selection::Selection;
use crate::task::workspace::Workspace;

pub struct Wasm;

impl Wasm {
    const CRATES: &[&str] = &["board", "eval", "fen", "search", "uci"];

    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        Self::CRATES
            .iter()
            .filter(|crate_name| {
                crate_name
                    .parse()
                    .is_ok_and(|name| selection.includes(&name))
            })
            .try_for_each(|crate_name| {
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
