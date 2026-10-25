use super::CastlingSquares;
use crate::leaper::King;
use crate::slider::Rook;
use crate::square::Square;

pub trait Castler {
    fn origin(squares: CastlingSquares) -> Square;

    fn destination(squares: CastlingSquares) -> Square;
}

impl Castler for King {
    fn origin(squares: CastlingSquares) -> Square {
        squares.king_origin
    }

    fn destination(squares: CastlingSquares) -> Square {
        squares.king_destination
    }
}

impl Castler for Rook {
    fn origin(squares: CastlingSquares) -> Square {
        squares.rook_origin
    }

    fn destination(squares: CastlingSquares) -> Square {
        squares.rook_destination
    }
}
