use super::tool::Tool;
use crate::task::command_line::CargoArgument;

pub struct CargoTool;

impl Tool for CargoTool {
    fn whole_workspace() -> Option<CargoArgument> {
        Some(CargoArgument::Workspace)
    }
}
