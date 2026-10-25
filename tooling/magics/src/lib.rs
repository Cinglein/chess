mod magic_tables;
mod xor_shift;

use magic_tables::MagicTables;

use crate::xor_shift::XorShift;
use task::Failure;
use task::Workspace;
use task::{CargoArgument, Program};

pub struct Magics;

impl Magics {
    const BOARD: &str = "board";

    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        MagicTables::find(&mut XorShift::new(MagicTables::SEED)).write(workspace.root())?;
        workspace
            .line(Program::Cargo)
            .argument(CargoArgument::Fmt)
            .argument(CargoArgument::Package)
            .argument(Self::BOARD)
            .run()
    }
}
