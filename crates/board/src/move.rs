use core::fmt;

use strum::VariantArray;

use crate::move_kind::MoveKind;
use crate::promotion::Promotion;
use crate::square::Square;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move(u16);

impl Move {
    const SQUARE_MASK: u16 = 0b11_1111;
    const TO_SHIFT: u16 = 6;
    const KIND_SHIFT: u16 = 12;
    const PROMOTION_SHIFT: u16 = 14;

    #[must_use]
    pub const fn new(from: Square, to: Square, kind: MoveKind) -> Move {
        let (kind_code, promotion_code) = match kind {
            MoveKind::Normal => (0, 0),
            MoveKind::Promotion(promotion) => (1, promotion as u16),
            MoveKind::EnPassant => (2, 0),
            MoveKind::Castling => (3, 0),
        };
        Move(
            from as u16
                | (to as u16) << Self::TO_SHIFT
                | kind_code << Self::KIND_SHIFT
                | promotion_code << Self::PROMOTION_SHIFT,
        )
    }

    #[must_use]
    pub fn from(self) -> Square {
        Square::VARIANTS[usize::from(self.0 & Self::SQUARE_MASK)]
    }

    #[must_use]
    pub fn to(self) -> Square {
        Square::VARIANTS[usize::from((self.0 >> Self::TO_SHIFT) & Self::SQUARE_MASK)]
    }

    #[must_use]
    pub fn kind(self) -> MoveKind {
        match (self.0 >> Self::KIND_SHIFT) & 0b11 {
            0 => MoveKind::Normal,
            1 => MoveKind::Promotion(
                Promotion::VARIANTS[usize::from(self.0 >> Self::PROMOTION_SHIFT)],
            ),
            2 => MoveKind::EnPassant,
            _ => MoveKind::Castling,
        }
    }

    #[must_use]
    pub fn promotion(self) -> Option<Promotion> {
        match self.kind() {
            MoveKind::Promotion(promotion) => Some(promotion),
            _ => None,
        }
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Move({self}, {:?})", self.kind())
    }
}

impl fmt::Display for Move {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.from(), self.to())?;
        if let Some(promotion) = self.promotion() {
            write!(formatter, "{promotion}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Move;
    use crate::move_kind::MoveKind;
    use crate::promotion::Promotion;
    use crate::square::Square;

    #[test]
    fn a_move_packs_into_sixteen_bits_and_unpacks_unchanged() {
        assert_eq!(size_of::<Move>(), 2);
        let kinds = [MoveKind::Normal, MoveKind::EnPassant, MoveKind::Castling]
            .into_iter()
            .chain(Promotion::iter().map(MoveKind::Promotion));
        for kind in kinds {
            let mv = Move::new(Square::H8, Square::A1, kind);
            assert_eq!(mv.from(), Square::H8);
            assert_eq!(mv.to(), Square::A1);
            assert_eq!(mv.kind(), kind);
        }
    }

    #[test]
    fn moves_display_in_long_algebraic_notation() {
        assert_eq!(
            Move::new(Square::E2, Square::E4, MoveKind::Normal).to_string(),
            "e2e4"
        );
        assert_eq!(
            Move::new(Square::E1, Square::G1, MoveKind::Castling).to_string(),
            "e1g1"
        );
        assert_eq!(
            Move::new(
                Square::E7,
                Square::E8,
                MoveKind::Promotion(Promotion::Queen)
            )
            .to_string(),
            "e7e8q"
        );
    }
}
