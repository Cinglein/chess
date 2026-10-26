use board::PieceKind;
use enum_map::EnumMap;

use crate::score::Score;

pub struct PieceKindValue;

impl PieceKindValue {
    const MATERIAL: EnumMap<PieceKind, Score> =
        EnumMap::from_array(Score::table([100, 320, 330, 500, 900, 20_000]));

    #[must_use]
    pub fn material(kind: PieceKind) -> Score {
        Self::MATERIAL[kind]
    }
}
