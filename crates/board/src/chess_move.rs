use core::fmt;

use crate::castling_right::CastlingRight;
use crate::promotion::Promotion;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ChessMove {
    Normal {
        from: Square,
        to: Square,
    },
    Promotion {
        from: Square,
        to: Square,
        piece: Promotion,
    },
    EnPassant {
        from: Square,
        to: Square,
    },
    Castling(CastlingRight),
}

impl ChessMove {
    #[must_use]
    pub const fn from(self) -> Square {
        match self {
            ChessMove::Normal { from, .. }
            | ChessMove::Promotion { from, .. }
            | ChessMove::EnPassant { from, .. } => from,
            ChessMove::Castling(right) => right.castling().king_from,
        }
    }

    #[must_use]
    pub const fn to(self) -> Square {
        match self {
            ChessMove::Normal { to, .. }
            | ChessMove::Promotion { to, .. }
            | ChessMove::EnPassant { to, .. } => to,
            ChessMove::Castling(right) => right.castling().king_to,
        }
    }
}

impl fmt::Display for ChessMove {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.from(), self.to())?;
        match self {
            ChessMove::Promotion { piece, .. } => write!(formatter, "{piece}"),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChessMove;
    use crate::castling_right::CastlingRight;
    use crate::promotion::Promotion;
    use crate::square::Square;

    const DISPLAYED: [(ChessMove, &str); 2] = [
        (ChessMove::Castling(CastlingRight::WhiteKingside), "e1g1"),
        (
            ChessMove::Promotion {
                from: Square::E7,
                to: Square::E8,
                piece: Promotion::Queen,
            },
            "e7e8q",
        ),
    ];

    #[test]
    fn moves_display_in_long_algebraic_notation() {
        for (chess_move, text) in DISPLAYED {
            assert_eq!(chess_move.to_string(), text);
        }
    }
}
