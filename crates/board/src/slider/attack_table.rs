use core::sync::atomic::{AtomicU64, Ordering};

use enum_map::EnumMap;
use strum::{EnumCount, VariantArray};

use super::{Magic, Rays};
use crate::bitboard::Bitboard;
use crate::square::Square;

pub(super) struct AttackTable<const SIZE: usize> {
    rays: Rays,
    magics: EnumMap<Square, Magic>,
    attack_slots: [AtomicU64; SIZE],
}

impl<const SIZE: usize> AttackTable<SIZE> {
    pub(super) const fn new(rays: Rays, multipliers: &EnumMap<Square, u64>) -> Self {
        AttackTable {
            rays,
            magics: Self::magics(rays, multipliers),
            attack_slots: [const { AtomicU64::new(0) }; SIZE],
        }
    }

    pub(super) fn attacks(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let slot = &self.attack_slots[self.magics[square].index(occupied)];
        match slot.load(Ordering::Relaxed) {
            0 => {
                let attacks = self.rays.attacks_by_ray(square, occupied);
                slot.store(attacks.bits(), Ordering::Relaxed);
                attacks
            }
            cached => Bitboard::from_bits(cached),
        }
    }

    const fn magics(rays: Rays, multipliers: &EnumMap<Square, u64>) -> EnumMap<Square, Magic> {
        let mut magics = [Magic::new(Bitboard::EMPTY, 0, 0); Square::COUNT];
        let mut offset = 0;
        let mut squares = Square::VARIANTS;
        while let [square, rest @ ..] = squares {
            let mask = rays.relevant_occupancy(*square);
            let magic = Magic::new(mask, multipliers.as_array()[*square as usize], offset);
            magics[*square as usize] = magic;
            offset += magic.table_size();
            squares = rest;
        }
        EnumMap::from_array(magics)
    }
}
