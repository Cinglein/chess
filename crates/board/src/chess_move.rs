use core::fmt;

use bitfield_struct::bitfield;

use crate::move_kind::MoveKind;
use crate::promotion::Promotion;
use crate::square::Square;

#[bitfield(u16, new = false, debug = false, default = false)]
#[derive(PartialEq, Eq, Hash)]
pub struct ChessMove {
    #[bits(6)]
    pub from: Square,
    #[bits(6)]
    pub to: Square,
    #[bits(2)]
    pub kind: MoveKind,
    #[bits(2)]
    piece: Promotion,
}

impl ChessMove {
    #[must_use]
    pub const fn normal(from: Square, to: Square) -> ChessMove {
        Self::of_kind(from, to, MoveKind::Normal)
    }

    #[must_use]
    pub const fn en_passant(from: Square, to: Square) -> ChessMove {
        Self::of_kind(from, to, MoveKind::EnPassant)
    }

    #[must_use]
    pub const fn castling(from: Square, to: Square) -> ChessMove {
        Self::of_kind(from, to, MoveKind::Castling)
    }

    #[must_use]
    pub const fn promotion(from: Square, to: Square, piece: Promotion) -> ChessMove {
        Self::of_kind(from, to, MoveKind::Promotion).with_piece(piece)
    }

    #[must_use]
    pub const fn promotion_piece(&self) -> Option<Promotion> {
        match self.kind() {
            MoveKind::Promotion => Some(self.piece()),
            _ => None,
        }
    }

    const fn of_kind(from: Square, to: Square, kind: MoveKind) -> ChessMove {
        ChessMove(0).with_from(from).with_to(to).with_kind(kind)
    }
}

impl fmt::Debug for ChessMove {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "ChessMove({self}, {:?})", self.kind())
    }
}

impl fmt::Display for ChessMove {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.from(), self.to())?;
        if let Some(piece) = self.promotion_piece() {
            write!(formatter, "{piece}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::ChessMove;
    use crate::move_kind::MoveKind;
    use crate::promotion::Promotion;
    use crate::square::Square;

    #[test]
    fn a_move_packs_into_sixteen_bits_and_unpacks_unchanged() {
        assert_eq!(size_of::<ChessMove>(), 2);
        let plain = [
            ChessMove::normal(Square::H8, Square::A1),
            ChessMove::en_passant(Square::H8, Square::A1),
            ChessMove::castling(Square::H8, Square::A1),
        ];
        let promotions =
            Promotion::iter().map(|piece| ChessMove::promotion(Square::H8, Square::A1, piece));
        for mv in plain.into_iter().chain(promotions) {
            assert_eq!(mv.from(), Square::H8);
            assert_eq!(mv.to(), Square::A1);
        }
        assert_eq!(plain[1].kind(), MoveKind::EnPassant);
        assert_eq!(plain[1].promotion_piece(), None);
        let queening = ChessMove::promotion(Square::E7, Square::E8, Promotion::Queen);
        assert_eq!(queening.kind(), MoveKind::Promotion);
        assert_eq!(queening.promotion_piece(), Some(Promotion::Queen));
    }

    #[test]
    fn moves_display_in_long_algebraic_notation() {
        assert_eq!(
            ChessMove::normal(Square::E2, Square::E4).to_string(),
            "e2e4"
        );
        assert_eq!(
            ChessMove::castling(Square::E1, Square::G1).to_string(),
            "e1g1"
        );
        assert_eq!(
            ChessMove::promotion(Square::E7, Square::E8, Promotion::Queen).to_string(),
            "e7e8q"
        );
    }
}
