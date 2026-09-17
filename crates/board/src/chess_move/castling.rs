use super::MoveKind;
use crate::castling_right::CastlingRight;
use crate::castling_squares::CastlingSquares;
use crate::placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Castling(CastlingRight);

impl Castling {
    #[must_use]
    pub const fn new(right: CastlingRight) -> Castling {
        Castling(right)
    }
}

impl MoveKind for Castling {
    fn origin(&self) -> Square {
        CastlingSquares::new(self.0).king_origin()
    }

    fn destination(&self) -> Square {
        CastlingSquares::new(self.0).king_destination()
    }

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement> {
        let squares = CastlingSquares::new(self.0);
        Some(
            placement
                .lift(squares.king_origin())?
                .land(squares.king_destination())
                .lift(squares.rook_origin())?
                .land(squares.rook_destination()),
        )
    }
}
