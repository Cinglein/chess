use enum_map::EnumMap;
use strum::{EnumCount, VariantArray};

use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::square::Square;

pub struct Leaps(&'static [&'static [Direction]]);

impl Leaps {
    #[must_use]
    pub const fn new(jumps: &'static [&'static [Direction]]) -> Leaps {
        Leaps(jumps)
    }

    #[must_use]
    pub const fn attacks_from(&self, origin: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let mut jump = 0;
        while jump < self.0.len() {
            let mut target = origin;
            let mut step = 0;
            while step < self.0[jump].len() {
                target = target.shift(self.0[jump][step]);
                step += 1;
            }
            attacks = attacks.union(target);
            jump += 1;
        }
        attacks
    }

    #[must_use]
    pub const fn table(&self) -> EnumMap<Square, Bitboard> {
        let mut table = [Bitboard::EMPTY; Square::COUNT];
        let mut index = 0;
        while index < Square::COUNT {
            table[index] = self.attacks_from(Bitboard::from_square(Square::VARIANTS[index]));
            index += 1;
        }
        EnumMap::from_array(table)
    }
}
