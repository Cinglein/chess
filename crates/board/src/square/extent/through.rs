use super::Extent;
use crate::bitboard::{Bitboard, SetOperation};
use crate::slider::{Bishop, Rook, Slider};
use crate::square::Square;

pub struct Through;

impl Extent for Through {
    fn trace(from: Square, to: Square) -> Bitboard {
        let ends =
            Bitboard::from_square(from).combined(Bitboard::from_square(to), SetOperation::Union);
        from.sight::<Rook>(to)
            .map(|sight| sight & Rook::attacks(to, Bitboard::EMPTY))
            .or_else(|| {
                from.sight::<Bishop>(to)
                    .map(|sight| sight & Bishop::attacks(to, Bitboard::EMPTY))
            })
            .map_or(Bitboard::EMPTY, |line| {
                line.combined(ends, SetOperation::DisjointUnion)
            })
    }
}
