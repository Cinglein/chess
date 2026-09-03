use crate::bitboard::Bitboard;

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
        let subset = self.next?;
        let following = self.mask.subset_after(subset);
        self.next = if following.is_empty() {
            None
        } else {
            Some(following)
        };
        Some(subset)
    }
}
