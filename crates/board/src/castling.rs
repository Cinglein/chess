use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Castling {
    pub king_from: Square,
    pub king_to: Square,
    pub rook_from: Square,
    pub rook_to: Square,
}
