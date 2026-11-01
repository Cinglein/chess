mod bits;

pub use bits::{HexLiteral, MagicSearch, MagicTables, XorShift};

use task::Failure;
use task::Workspace;
use task::{CargoArgument, Program};

pub struct Magics;

impl Magics {
    const BOARD: &str = "board";

    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        MagicTables::find(&mut XorShift::from(MagicTables::SEED)).write(workspace.root())?;
        workspace
            .line(Program::Cargo)
            .argument(CargoArgument::Fmt)
            .argument(CargoArgument::Package)
            .argument(Self::BOARD)
            .run()
    }
}
