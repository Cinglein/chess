use crate::task::command_line::{CargoWord, Program};
use crate::task::failure::Failure;
use crate::task::selection::Selection;
use crate::task::workspace::Workspace;

pub struct Test;

impl Test {
    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        if selection.is_empty() {
            println!("test: no crate affected");
            return Ok(());
        }
        workspace
            .line(Program::Cargo)
            .word(CargoWord::Test)
            .words(selection.package_words())
            .word(CargoWord::AllFeatures)
            .run()
    }
}
