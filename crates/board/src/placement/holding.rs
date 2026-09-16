use super::Hand;
use crate::piece::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Holding(Piece);

impl Holding {
    #[must_use]
    pub const fn new(piece: Piece) -> Holding {
        Holding(piece)
    }

    #[must_use]
    pub const fn piece(self) -> Piece {
        self.0
    }
}

impl Hand for Holding {}
