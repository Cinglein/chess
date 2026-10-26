mod wasm_argument;

use task::{CargoArgument, CrateName, Failure, Program, Selection, Workspace};
use wasm_argument::WasmArgument;

pub struct Wasm;

impl Wasm {
    const CRATES: &[&str] = &["board", "eval", "fen", "search", "uci"];

    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        Self::CRATES
            .iter()
            .filter_map(|crate_name| crate_name.parse::<CrateName>().ok())
            .filter(|crate_name| Self::selects(selection, crate_name))
            .try_for_each(|crate_name| {
                workspace
                    .line(Program::Cargo)
                    .argument(CargoArgument::Clippy)
                    .argument(CargoArgument::Package)
                    .argument(crate_name)
                    .argument(WasmArgument::Target)
                    .argument(WasmArgument::Wasm32)
                    .argument(CargoArgument::ToolArguments)
                    .argument(CargoArgument::Deny)
                    .argument(CargoArgument::Warnings)
                    .run()
            })
    }

    fn selects(selection: &Selection, crate_name: &CrateName) -> bool {
        if let Selection::Crates(crates) = selection {
            crates.contains(crate_name)
        } else {
            true
        }
    }
}
