use crate::bitboard::Bitboard;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Castling {
    pub king_origin: Square,
    pub king_destination: Square,
    pub rook_origin: Square,
    pub rook_destination: Square,
}

impl Castling {
    #[must_use]
    pub const fn footprint(self) -> Bitboard {
        Bitboard::from_square(self.king_origin).with(self.rook_origin)
    }
}
