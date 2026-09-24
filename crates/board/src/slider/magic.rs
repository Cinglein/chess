use crate::bitboard::Bitboard;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Magic {
    mask: Bitboard,
    multiplier: u64,
    shift: u32,
    offset: usize,
}

impl Magic {
    #[must_use]
    pub const fn new(mask: Bitboard, multiplier: u64, offset: usize) -> Magic {
        Magic {
            mask,
            multiplier,
            shift: u64::BITS - 1 - mask.count(),
            offset,
        }
    }

    #[must_use]
    pub const fn mask(&self) -> Bitboard {
        self.mask
    }

    #[must_use]
    pub const fn table_size(&self) -> usize {
        1 << self.mask.count()
    }

    #[must_use]
    pub const fn index(&self, occupied: Bitboard) -> usize {
        let hash = occupied
            .intersection(self.mask)
            .bits()
            .wrapping_mul(self.multiplier);
        self.offset + ((hash >> 1) >> self.shift) as usize
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::Magic;
    use crate::bitboard::Bitboard;
    use crate::rank::Rank;

    #[test]
    fn the_index_depends_only_on_the_mask_and_stays_inside_the_table() {
        proptest!(|(mask: u64, multiplier: u64, occupied: u64, offset in 0..64usize)| {
            let mask = Bitboard::from_bits(mask) & Bitboard::rank(Rank::One);
            let magic = Magic::new(mask, multiplier, offset);
            let occupied = Bitboard::from_bits(occupied);
            let index = magic.index(occupied);
            prop_assert!((offset..offset + magic.table_size()).contains(&index));
            prop_assert_eq!(magic.index(occupied & mask), index);
        });
    }
}
