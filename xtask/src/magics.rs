use crate::magic_tables::MagicTables;
use crate::workspace::Workspace;
use crate::xor_shift::XorShift;

pub struct Magics;

impl Magics {
    pub fn run(workspace: &Workspace) -> Result<(), String> {
        MagicTables::find(&mut XorShift::new(MagicTables::SEED)).write(workspace.root())?;
        workspace.cargo(&["fmt", "--package", "board"])
    }
}
