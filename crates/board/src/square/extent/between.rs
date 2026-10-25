use super::Extent;
use crate::bitboard::Bitboard;
use crate::slider::{Bishop, Rook, Slider};
use crate::square::Square;

pub struct Between;

impl Extent for Between {
    fn trace(from: Square, to: Square) -> Bitboard {
        let origin = Bitboard::from_square(from);
        let target = Bitboard::from_square(to);
        if from.aligned_by::<Rook>(to) {
            Rook::attacks(from, target) & Rook::attacks(to, origin)
        } else if from.aligned_by::<Bishop>(to) {
            Bishop::attacks(from, target) & Bishop::attacks(to, origin)
        } else {
            Bitboard::EMPTY
        }
    }
}
