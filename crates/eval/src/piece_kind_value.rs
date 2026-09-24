use board::PieceKind;
use enum_map::EnumMap;

use crate::score::Score;

pub struct PieceKindValue;

impl PieceKindValue {
    const CENTIPAWNS: EnumMap<PieceKind, i32> =
        EnumMap::from_array([100, 320, 330, 500, 900, 20_000]);

    #[must_use]
    pub fn material(kind: PieceKind) -> Score {
        Score::new(Self::CENTIPAWNS[kind])
    }
}
