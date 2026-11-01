use super::Hand;
use crate::piece::Piece;
use newtype::Newtype;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Newtype)]
pub struct Holding(Piece);

impl Holding {
    #[must_use]
    pub const fn piece(self) -> Piece {
        self.0
    }
}

impl Hand for Holding {}
