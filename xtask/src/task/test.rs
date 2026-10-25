use task::Failure;
use task::Workspace;
use task::{CargoArgument, Program};
use task::{CargoTool, Selection};

pub struct Test;

impl Test {
    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        if selection.is_empty() {
            println!("test: no crate affected");
            Ok(())
        } else {
            workspace
                .line(Program::Cargo)
                .argument(CargoArgument::Test)
                .argument(CargoArgument::Lib)
                .arguments(selection.arguments::<CargoTool>())
                .argument(CargoArgument::AllFeatures)
                .run()
        }
    }
}
