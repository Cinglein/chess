use super::PiecePlacement;
use crate::chess_move::ChessMove;
use crate::square::Square;

impl PiecePlacement {
    #[must_use]
    pub fn apply(self, chess_move: ChessMove) -> Option<PiecePlacement> {
        let lifted = self.lift(chess_move.origin())?;
        Some(match chess_move {
            ChessMove::Normal { destination, .. } => lifted.land(destination),
            ChessMove::Promotion {
                destination, piece, ..
            } => lifted.promote(piece).land(destination),
            ChessMove::EnPassant {
                origin,
                destination,
            } => {
                let passed = Square::new(destination.file(), origin.rank());
                lifted.land(passed).lift(passed)?.land(destination)
            }
            ChessMove::Castling(right) => {
                let castling = right.castling();
                lifted
                    .land(castling.king_destination)
                    .lift(castling.rook_origin)?
                    .land(castling.rook_destination)
            }
        })
    }
}
