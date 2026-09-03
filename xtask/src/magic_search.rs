use board::{Bitboard, Magic, Slider, Square};

use crate::xor_shift::XorShift;

pub struct MagicSearch {
    mask: Bitboard,
    expected: Vec<(Bitboard, Bitboard)>,
    table: Vec<Bitboard>,
}

impl MagicSearch {
    pub fn new(slider: Slider, square: Square) -> MagicSearch {
        let mask = slider.relevant_occupancy(square);
        let expected = mask
            .subsets()
            .map(|subset| (subset, slider.attacks_by_ray(square, subset)))
            .collect();
        let table = vec![Bitboard::EMPTY; Magic::new(mask, 0, 0).table_size()];
        MagicSearch {
            mask,
            expected,
            table,
        }
    }

    pub fn run(&mut self, rng: &mut XorShift) -> u64 {
        loop {
            let candidate = rng.sparse();
            if self.accepts(candidate) {
                return candidate;
            }
        }
    }

    fn accepts(&mut self, candidate: u64) -> bool {
        if !self.spreads_well(candidate) {
            return false;
        }
        let magic = Magic::new(self.mask, candidate, 0);
        self.table.fill(Bitboard::EMPTY);
        self.expected.iter().all(|&(subset, attacks)| {
            let slot = &mut self.table[magic.index(subset)];
            if slot.is_empty() {
                *slot = attacks;
                true
            } else {
                *slot == attacks
            }
        })
    }

    fn spreads_well(&self, candidate: u64) -> bool {
        (self.mask.bits().wrapping_mul(candidate) >> 56).count_ones() >= 6
    }
}

#[cfg(test)]
mod tests {
    use board::{Magic, Slider, Square};

    use super::MagicSearch;
    use crate::xor_shift::XorShift;

    #[test]
    fn a_found_magic_maps_every_occupancy_to_its_attacks() {
        let square = Square::D4;
        let multiplier = MagicSearch::new(Slider::Bishop, square).run(&mut XorShift::new(7));
        let mask = Slider::Bishop.relevant_occupancy(square);
        let magic = Magic::new(mask, multiplier, 0);
        let mut table = vec![None; magic.table_size()];
        for subset in mask.subsets() {
            let attacks = Slider::Bishop.attacks_by_ray(square, subset);
            let slot = &mut table[magic.index(subset)];
            assert!(slot.is_none_or(|stored| stored == attacks));
            *slot = Some(attacks);
        }
    }
}
