use std::fmt;
use std::fs;
use std::path::Path;

use board::{Bishop, Rook, Slider, Square};
use enum_map::EnumMap;

use crate::hex_literal::HexLiteral;
use crate::magic_search::MagicSearch;
use crate::xor_shift::XorShift;

pub struct MagicTables {
    rook: EnumMap<Square, u64>,
    bishop: EnumMap<Square, u64>,
}

impl MagicTables {
    pub const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
    const OUTPUT: &str = "crates/board/src/slider/magics.rs";

    pub fn find(rng: &mut XorShift) -> MagicTables {
        MagicTables {
            rook: Self::find_all::<Rook>(rng),
            bishop: Self::find_all::<Bishop>(rng),
        }
    }

    pub fn write(&self, workspace_root: &Path) -> Result<(), String> {
        let path = workspace_root.join(Self::OUTPUT);
        fs::write(&path, self.to_string()).map_err(|error| format!("{}: {error}", Self::OUTPUT))?;
        println!("wrote {}", Self::OUTPUT);
        Ok(())
    }

    fn find_all<S: Slider>(rng: &mut XorShift) -> EnumMap<Square, u64> {
        EnumMap::from_fn(|square| MagicSearch::new::<S>(square).run(rng))
    }

    fn fmt_table(
        formatter: &mut fmt::Formatter<'_>,
        name: &str,
        values: &EnumMap<Square, u64>,
    ) -> fmt::Result {
        writeln!(
            formatter,
            "pub(super) const {name}: EnumMap<Square, u64> = EnumMap::from_array(["
        )?;
        for value in values.values() {
            writeln!(formatter, "    {},", HexLiteral(*value))?;
        }
        writeln!(formatter, "]);")
    }
}

impl fmt::Display for MagicTables {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "use enum_map::EnumMap;\n\nuse crate::square::Square;\n"
        )?;
        Self::fmt_table(formatter, "ROOK", &self.rook)?;
        writeln!(formatter)?;
        Self::fmt_table(formatter, "BISHOP", &self.bishop)
    }
}
