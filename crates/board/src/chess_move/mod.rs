mod castling;
mod double_push;
mod en_passant;
mod move_kind;
mod normal;
mod promotion;

pub use castling::Castling;
pub use double_push::DoublePush;
pub use en_passant::EnPassant;
pub use move_kind::MoveKind;
pub use normal::Normal;
pub use promotion::Promotion;

use core::fmt;

use enum_dispatch::enum_dispatch;

use crate::file::File;
use crate::piece_placement::PiecePlacement;
use crate::promotion_piece::PromotionPiece;
use crate::square::Square;

#[enum_dispatch(MoveKind)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ChessMove {
    Normal(Normal),
    DoublePush(DoublePush),
    Promotion(Promotion),
    EnPassant(EnPassant),
    Castling(Castling),
}

impl fmt::Display for ChessMove {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.origin(), self.destination())?;
        self.promotion_piece()
            .map_or(Ok(()), |piece| write!(formatter, "{piece}"))
    }
}

#[cfg(test)]
mod tests {
    use super::{Castling, ChessMove, Promotion};
    use crate::castling_right::CastlingRight;
    use crate::promotion_piece::PromotionPiece;
    use crate::square::Square;

    const DISPLAYED: [(ChessMove, &str); 2] = [
        (
            ChessMove::Castling(Castling(CastlingRight::WhiteKingside)),
            "e1g1",
        ),
        (
            ChessMove::Promotion(Promotion {
                origin: Square::E7,
                destination: Square::E8,
                piece: PromotionPiece::Queen,
            }),
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
