use task::Failure;
use task::Workspace;
use task::{CargoArgument, Program};
use task::{CargoTool, Selection};

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
            .arguments(selection.arguments::<CargoTool>())
            .argument(CargoArgument::AllTargets)
            .argument(CargoArgument::AllFeatures)
            .argument(CargoArgument::ToolArguments)
            .argument(CargoArgument::Deny)
            .argument(CargoArgument::Warnings)
            .run()
    }
}
