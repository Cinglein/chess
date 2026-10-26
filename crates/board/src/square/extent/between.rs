use super::Extent;
use crate::bitboard::Bitboard;
use crate::slider::{Bishop, Rook, Slider};
use crate::square::Square;

pub struct Between;

impl Extent for Between {
    fn trace(from: Square, to: Square) -> Bitboard {
        let origin = Bitboard::from_square(from);
        let target = Bitboard::from_square(to);
        from.sight::<Rook>(to)
            .map(|_| Rook::attacks(from, target) & Rook::attacks(to, origin))
            .or_else(|| {
                from.sight::<Bishop>(to)
                    .map(|_| Bishop::attacks(from, target) & Bishop::attacks(to, origin))
            })
            .unwrap_or(Bitboard::EMPTY)
    }
}
