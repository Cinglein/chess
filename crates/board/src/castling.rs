use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Castling {
    pub king_from: Square,
    pub king_to: Square,
    pub rook_from: Square,
    pub rook_to: Square,
}

impl Castling {
    #[must_use]
    pub fn touches(self, square: Square) -> bool {
        self.king_from == square || self.rook_from == square
    }
}
