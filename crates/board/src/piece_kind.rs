use enum_map::Enum;
use strum::{Enum, EnumCount, EnumIter, FromRepr, VariantArray};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumCount,
    EnumIter,
    FromRepr,
    VariantArray,
)]
#[repr(u8)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    #[must_use]
    pub const fn letter(self) -> char {
        match self {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        }
    }

    #[must_use]
    pub const fn from_letter(letter: char) -> Option<PieceKind> {
        match letter.to_ascii_lowercase() {
            'p' => Some(PieceKind::Pawn),
            'n' => Some(PieceKind::Knight),
            'b' => Some(PieceKind::Bishop),
            'r' => Some(PieceKind::Rook),
            'q' => Some(PieceKind::Queen),
            'k' => Some(PieceKind::King),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::PieceKind;

    #[test]
    fn every_kind_letter_roundtrips_in_either_case() {
        for kind in PieceKind::iter() {
            assert_eq!(PieceKind::from_letter(kind.letter()), Some(kind));
            assert_eq!(
                PieceKind::from_letter(kind.letter().to_ascii_uppercase()),
                Some(kind)
            );
        }
        assert_eq!(PieceKind::from_letter('x'), None);
    }
}
