use super::Bitboard;

#[derive(Clone, Debug)]
pub struct SubsetIter {
    mask: Bitboard,
    next: Option<Bitboard>,
}

impl SubsetIter {
    #[must_use]
    pub const fn new(mask: Bitboard) -> SubsetIter {
        SubsetIter {
            mask,
            next: Some(Bitboard::EMPTY),
        }
    }
}

impl Iterator for SubsetIter {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Bitboard> {
        let subset = self.next.take()?;
        let following =
            Bitboard::from_bits(subset.bits().wrapping_sub(self.mask.bits()) & self.mask.bits());
        self.next = Some(following).filter(|following| !following.is_empty());
        Some(subset)
    }
}
