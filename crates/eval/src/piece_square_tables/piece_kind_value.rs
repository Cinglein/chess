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

#[cfg(test)]
mod tests {
    use board::PieceKind;

    use super::PieceKindValue;

    #[test]
    fn heavier_pieces_are_worth_more_material() {
        assert!(
            PieceKindValue::material(PieceKind::Queen) > PieceKindValue::material(PieceKind::Rook)
        );
        assert!(
            PieceKindValue::material(PieceKind::Rook) > PieceKindValue::material(PieceKind::Pawn)
        );
    }
}
