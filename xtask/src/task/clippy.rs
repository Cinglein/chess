use crate::task::command_line::{CargoWord, Program};
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
            .word(CargoWord::Clippy)
            .words(selection.package_words())
            .word(CargoWord::AllTargets)
            .word(CargoWord::AllFeatures)
            .word(CargoWord::ToolArguments)
            .word(CargoWord::Deny)
            .word(CargoWord::Warnings)
            .run()
    }
}
