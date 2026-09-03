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
            shift: u64::BITS - mask.count(),
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the hash is shifted down to at most twelve bits before the cast"
    )]
    pub const fn index(&self, occupied: Bitboard) -> usize {
        let hash = occupied
            .intersection(self.mask)
            .bits()
            .wrapping_mul(self.multiplier);
        let slot = match hash.checked_shr(self.shift) {
            Some(slot) => slot,
            None => 0,
        };
        self.offset + slot as usize
    }
}

#[cfg(test)]
mod tests {
    use super::Magic;
    use crate::bitboard::Bitboard;
    use crate::square::Square;

    #[test]
    fn the_index_depends_only_on_the_mask_and_stays_inside_the_table() {
        let mask: Bitboard = [Square::B1, Square::C1, Square::A2, Square::A3]
            .into_iter()
            .collect();
        let noise: Bitboard = [Square::D4, Square::H8].into_iter().collect();
        let magic = Magic::new(mask, 0x9E37_79B9_7F4A_7C15, 100);
        assert_eq!(magic.table_size(), 16);
        for occupied in mask.subsets() {
            let index = magic.index(occupied);
            assert!((100..116).contains(&index), "{index}");
            assert_eq!(magic.index(occupied | noise), index);
        }
    }
}
