use crate::bitboard::Bitboard;
use crate::square::Square;

#[derive(Clone, Debug)]
pub struct Squares(Bitboard);

impl Squares {
    #[must_use]
    pub const fn new(bitboard: Bitboard) -> Squares {
        Squares(bitboard)
    }
}

impl Iterator for Squares {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        let square = self.0.first()?;
        self.0 = self.0.without_first();
        Some(square)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.0.count() as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Squares {}
