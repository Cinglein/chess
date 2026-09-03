use core::fmt;

use crate::piece::PieceKind;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Promotion {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl Promotion {
    pub const ALL: [Promotion; 4] = [
        Promotion::Knight,
        Promotion::Bishop,
        Promotion::Rook,
        Promotion::Queen,
    ];

    #[must_use]
    pub const fn piece_kind(self) -> PieceKind {
        match self {
            Promotion::Knight => PieceKind::Knight,
            Promotion::Bishop => PieceKind::Bishop,
            Promotion::Rook => PieceKind::Rook,
            Promotion::Queen => PieceKind::Queen,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoveKind {
    Normal,
    Promotion(Promotion),
    EnPassant,
    Castling,
}

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
    pub const fn from(self) -> Square {
        Square::ALL[(self.0 & Self::SQUARE_MASK) as usize]
    }

    #[must_use]
    pub const fn to(self) -> Square {
        Square::ALL[((self.0 >> Self::TO_SHIFT) & Self::SQUARE_MASK) as usize]
    }

    #[must_use]
    pub const fn kind(self) -> MoveKind {
        match (self.0 >> Self::KIND_SHIFT) & 0b11 {
            0 => MoveKind::Normal,
            1 => MoveKind::Promotion(Promotion::ALL[(self.0 >> Self::PROMOTION_SHIFT) as usize]),
            2 => MoveKind::EnPassant,
            _ => MoveKind::Castling,
        }
    }

    #[must_use]
    pub const fn promotion(self) -> Option<Promotion> {
        match self.kind() {
            MoveKind::Promotion(promotion) => Some(promotion),
            _ => None,
        }
    }

    #[must_use]
    pub const fn bits(self) -> u16 {
        self.0
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
            write!(formatter, "{}", promotion.piece_kind().letter())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Move, MoveKind, Promotion};
    use crate::square::Square;

    #[test]
    fn a_move_packs_into_sixteen_bits_and_unpacks_unchanged() {
        assert_eq!(size_of::<Move>(), 2);
        let kinds = [
            MoveKind::Normal,
            MoveKind::EnPassant,
            MoveKind::Castling,
            MoveKind::Promotion(Promotion::Knight),
            MoveKind::Promotion(Promotion::Bishop),
            MoveKind::Promotion(Promotion::Rook),
            MoveKind::Promotion(Promotion::Queen),
        ];
        for kind in kinds {
            let mv = Move::new(Square::A1, Square::H8, kind);
            assert_eq!(mv.from(), Square::A1);
            assert_eq!(mv.to(), Square::H8);
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
