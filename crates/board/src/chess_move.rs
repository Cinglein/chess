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

const _: () = assert!(size_of::<ChessMove>() == 2);

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
    use proptest::prelude::*;
    use proptest::sample::select;
    use strum::VariantArray;

    use super::ChessMove;
    use crate::move_kind::MoveKind;
    use crate::promotion::Promotion;
    use crate::square::Square;

    const DISPLAYED: [(ChessMove, &str); 3] = [
        (ChessMove::normal(Square::E2, Square::E4), "e2e4"),
        (ChessMove::castling(Square::E1, Square::G1), "e1g1"),
        (
            ChessMove::promotion(Square::E7, Square::E8, Promotion::Queen),
            "e7e8q",
        ),
    ];

    #[test]
    fn every_move_unpacks_to_what_it_was_built_from() {
        let squares = || select(Square::VARIANTS);
        proptest!(|(from in squares(), to in squares(), kind in select(MoveKind::VARIANTS), piece in select(Promotion::VARIANTS))| {
            let chess_move = match kind {
                MoveKind::Normal => ChessMove::normal(from, to),
                MoveKind::Promotion => ChessMove::promotion(from, to, piece),
                MoveKind::EnPassant => ChessMove::en_passant(from, to),
                MoveKind::Castling => ChessMove::castling(from, to),
            };
            prop_assert_eq!((chess_move.from(), chess_move.to(), chess_move.kind()), (from, to, kind));
            prop_assert_eq!(chess_move.promotion_piece(), (kind == MoveKind::Promotion).then_some(piece));
        });
    }

    #[test]
    fn moves_display_in_long_algebraic_notation() {
        for (chess_move, text) in DISPLAYED {
            assert_eq!(chess_move.to_string(), text);
        }
    }
}
