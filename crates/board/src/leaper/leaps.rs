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
        let mut jumps = self.0;
        while let [jump, rest @ ..] = jumps {
            attacks = attacks.union(Self::leap(origin, jump));
            jumps = rest;
        }
        attacks
    }

    #[must_use]
    pub const fn table(&self) -> EnumMap<Square, Bitboard> {
        let mut table = [Bitboard::EMPTY; Square::COUNT];
        let mut squares = Square::VARIANTS;
        while let [square, rest @ ..] = squares {
            table[*square as usize] = self.attacks_from(Bitboard::from_square(*square));
            squares = rest;
        }
        EnumMap::from_array(table)
    }

    const fn leap(origin: Bitboard, steps: &[Direction]) -> Bitboard {
        let mut target = origin;
        let mut steps = steps;
        while let [step, rest @ ..] = steps {
            target = target.shift(*step);
            steps = rest;
        }
        target
    }
}
