mod magic_tables;
mod xor_shift;

use magic_tables::MagicTables;

use crate::task::command_line::{CargoWord, Program};
use crate::task::failure::Failure;
use crate::task::magics::xor_shift::XorShift;
use crate::task::workspace::Workspace;

pub struct Magics;

impl Magics {
    const BOARD: &str = "board";

    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        MagicTables::find(&mut XorShift::new(MagicTables::SEED)).write(workspace.root())?;
        workspace
            .line(Program::Cargo)
            .word(CargoWord::Fmt)
            .word(CargoWord::Package)
            .word(Self::BOARD)
            .run()
    }
}
