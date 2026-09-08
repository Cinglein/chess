use core::fmt;

use crate::castling_right::CastlingRight;
use crate::promotion::Promotion;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ChessMove {
    Normal {
        origin: Square,
        destination: Square,
    },
    Promotion {
        origin: Square,
        destination: Square,
        piece: Promotion,
    },
    EnPassant {
        origin: Square,
        destination: Square,
    },
    Castling(CastlingRight),
}

impl ChessMove {
    #[must_use]
    pub const fn origin(self) -> Square {
        match self {
            ChessMove::Normal { origin, .. }
            | ChessMove::Promotion { origin, .. }
            | ChessMove::EnPassant { origin, .. } => origin,
            ChessMove::Castling(right) => right.castling().king_origin,
        }
    }

    #[must_use]
    pub const fn destination(self) -> Square {
        match self {
            ChessMove::Normal { destination, .. }
            | ChessMove::Promotion { destination, .. }
            | ChessMove::EnPassant { destination, .. } => destination,
            ChessMove::Castling(right) => right.castling().king_destination,
        }
    }
}

impl fmt::Display for ChessMove {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.origin(), self.destination())?;
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
                origin: Square::E7,
                destination: Square::E8,
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
