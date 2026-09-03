use crate::bitboard::Bitboard;
use crate::square::Square;

#[derive(Clone, Debug)]
pub struct SquareIter(Bitboard);

impl SquareIter {
    #[must_use]
    pub const fn new(bitboard: Bitboard) -> SquareIter {
        SquareIter(bitboard)
    }
}

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        let square = self.0.least_significant_bit()?;
        self.0 = self.0.without_least_significant_bit();
        Some(square)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.0.count() as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for SquareIter {}
