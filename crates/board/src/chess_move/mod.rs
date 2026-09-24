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
use crate::long_algebraic::LongAlgebraic;
use crate::piece::Piece;
use crate::placement::PiecePlacement;
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
        LongAlgebraic::from(*self).fmt(formatter)
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
            ChessMove::Castling(Castling::new(CastlingRight::WhiteKingside)),
            "e1g1",
        ),
        (
            ChessMove::Promotion(Promotion::new(
                Square::E7,
                Square::E8,
                PromotionPiece::Queen,
            )),
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
