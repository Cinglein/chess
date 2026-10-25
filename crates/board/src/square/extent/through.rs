use super::Extent;
use crate::bitboard::{Bitboard, SetOperation};
use crate::slider::{Bishop, Rook, Slider};
use crate::square::Square;

pub struct Through;

impl Extent for Through {
    fn trace(from: Square, to: Square) -> Bitboard {
        let ends =
            Bitboard::from_square(from).combined(Bitboard::from_square(to), SetOperation::Union);
        if from.aligned_by::<Rook>(to) {
            (Rook::attacks(from, Bitboard::EMPTY) & Rook::attacks(to, Bitboard::EMPTY))
                .combined(ends, SetOperation::DisjointUnion)
        } else if from.aligned_by::<Bishop>(to) {
            (Bishop::attacks(from, Bitboard::EMPTY) & Bishop::attacks(to, Bitboard::EMPTY))
                .combined(ends, SetOperation::DisjointUnion)
        } else {
            Bitboard::EMPTY
        }
    }
}
