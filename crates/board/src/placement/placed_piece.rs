use crate::piece::Piece;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlacedPiece {
    square: Square,
    piece: Piece,
}

impl PlacedPiece {
    #[must_use]
    pub const fn new(square: Square, piece: Piece) -> PlacedPiece {
        PlacedPiece { square, piece }
    }

    #[must_use]
    pub const fn square(self) -> Square {
        self.square
    }

    #[must_use]
    pub const fn piece(self) -> Piece {
        self.piece
    }
}
