use super::MoveKind;
use crate::castling_right::CastlingRight;
use crate::piece_placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Castling(pub CastlingRight);

impl MoveKind for Castling {
    fn origin(&self) -> Square {
        self.0.squares().king_origin
    }

    fn destination(&self) -> Square {
        self.0.squares().king_destination
    }

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement> {
        let squares = self.0.squares();
        Some(
            placement
                .lift(squares.king_origin)?
                .land(squares.king_destination)
                .lift(squares.rook_origin)?
                .land(squares.rook_destination),
        )
    }
}
