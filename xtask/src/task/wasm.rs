use crate::task::command_line::{CargoWord, Program};
use crate::task::crate_name::CrateName;
use crate::task::failure::Failure;
use crate::task::selection::Selection;
use crate::task::workspace::Workspace;

pub struct Wasm;

impl Wasm {
    const CRATES: &[&str] = &["board", "eval", "fen", "search", "uci"];

    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        Self::CRATES
            .iter()
            .filter_map(|crate_name| crate_name.parse::<CrateName>().ok())
            .filter(|crate_name| selection.includes(crate_name))
            .try_for_each(|crate_name| {
                workspace
                    .line(Program::Cargo)
                    .word(CargoWord::Clippy)
                    .word(CargoWord::Package)
                    .word(crate_name)
                    .word(CargoWord::Target)
                    .word(CargoWord::Wasm32)
                    .word(CargoWord::ToolArguments)
                    .word(CargoWord::Deny)
                    .word(CargoWord::Warnings)
                    .run()
            })
    }
}
