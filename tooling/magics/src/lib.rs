mod bits;

pub use bits::{HexLiteral, MagicSearch, MagicTables, XorShift};

use std::io;
use std::path::Path;

pub struct Magics;

impl Magics {
    pub fn run(workspace_root: &Path) -> io::Result<()> {
        MagicTables::find(&mut XorShift::from(MagicTables::SEED)).write(workspace_root)
    }
}
