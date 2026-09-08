use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Castling {
    pub king_origin: Square,
    pub king_destination: Square,
    pub rook_origin: Square,
    pub rook_destination: Square,
}
