use std::iter;

use board::{Bitboard, Magic, Slider, Square, SubsetIter};

use crate::xor_shift::XorShift;

pub struct MagicSearch {
    mask: Bitboard,
    attacks: Vec<Bitboard>,
    table: Vec<Bitboard>,
}

impl MagicSearch {
    pub fn new<S: Slider>(square: Square) -> MagicSearch {
        let mask = S::relevant_occupancy(square);
        let attacks = SubsetIter::new(mask)
            .map(|subset| S::attacks_by_ray(square, subset))
            .collect();
        let table = vec![Bitboard::EMPTY; Magic::new(mask, 0, 0).table_size()];
        MagicSearch { mask, attacks, table }
    }

    pub fn find_multiplier(&mut self, rng: &mut XorShift) -> u64 {
        iter::repeat_with(|| rng.sparse())
            .find(|candidate| self.accepts(*candidate))
            .expect("an endless stream of candidates holds a magic")
    }

    fn accepts(&mut self, candidate: u64) -> bool {
        self.spreads_well(candidate) && self.fills_all(Magic::new(self.mask, candidate, 0))
    }

    fn fills_all(&mut self, magic: Magic) -> bool {
        self.table.fill(Bitboard::EMPTY);
        let table = &mut self.table;
        SubsetIter::new(self.mask)
            .zip(&self.attacks)
            .all(|(subset, attacks)| Self::fills(table, magic.index(subset), *attacks))
    }

    fn fills(table: &mut [Bitboard], index: usize, attacks: Bitboard) -> bool {
        table.get_mut(index).is_some_and(|slot| {
            *slot = if slot.is_empty() { attacks } else { *slot };
            *slot == attacks
        })
    }

    fn spreads_well(&self, candidate: u64) -> bool {
        (self.mask.bits().wrapping_mul(candidate) >> 56).count_ones() >= 6
    }
}

#[cfg(test)]
mod tests {
    use board::{Bishop, Magic, Slider, Square, SubsetIter};

    use super::MagicSearch;
    use crate::xor_shift::XorShift;

    #[test]
    fn a_found_magic_maps_every_occupancy_to_its_attacks() {
        let square = Square::D4;
        let multiplier = MagicSearch::new::<Bishop>(square).find_multiplier(&mut XorShift::new(7));
        let mask = Bishop::relevant_occupancy(square);
        let magic = Magic::new(mask, multiplier, 0);
        let mut table = vec![None; magic.table_size()];
        for subset in SubsetIter::new(mask) {
            let attacks = Bishop::attacks_by_ray(square, subset);
            let slot = &mut table[magic.index(subset)];
            assert!(slot.is_none_or(|stored| stored == attacks));
            *slot = Some(attacks);
        }
    }
}
