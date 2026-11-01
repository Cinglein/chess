use crate::bitboard::Bitboard;
use crate::bitboard::SetOperation;

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
            .combined(self.mask, SetOperation::Intersection)
            .bits()
            .wrapping_mul(self.multiplier);
        self.offset + ((hash >> 1) >> self.shift) as usize
    }
}
