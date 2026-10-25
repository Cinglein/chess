use super::tool::Tool;
use crate::task::command_line::CargoArgument;

pub struct MutationTool;

impl Tool for MutationTool {
    fn whole_workspace() -> Option<CargoArgument> {
        None
    }
}
