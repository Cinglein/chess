use crate::task::command_line::{CargoArgument, Program};
use crate::task::failure::Failure;
use crate::task::selection::Selection;
use crate::task::workspace::Workspace;

pub struct Clippy;

impl Clippy {
    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        if selection.is_empty() {
            println!("clippy: no crate affected");
            return Ok(());
        }
        workspace
            .line(Program::Cargo)
            .argument(CargoArgument::Clippy)
            .arguments(selection.package_arguments())
            .argument(CargoArgument::AllTargets)
            .argument(CargoArgument::AllFeatures)
            .argument(CargoArgument::ToolArguments)
            .argument(CargoArgument::Deny)
            .argument(CargoArgument::Warnings)
            .run()
    }
}
