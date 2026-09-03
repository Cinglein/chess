use enum_map::EnumMap;
use strum::{EnumCount, VariantArray};

use super::Leaper;
use crate::bitboard::Bitboard;
use crate::square::Square;

pub(super) static ATTACKS: EnumMap<Leaper, EnumMap<Square, Bitboard>> = EnumMap::from_array([
    attack_table(Leaper::Knight),
    attack_table(Leaper::King),
    attack_table(Leaper::WhitePawn),
    attack_table(Leaper::BlackPawn),
]);

const fn attack_table(leaper: Leaper) -> EnumMap<Square, Bitboard> {
    let mut table = [Bitboard::EMPTY; Square::COUNT];
    let mut index = 0;
    while index < Square::COUNT {
        table[index] = leaper.attacks_from(Bitboard::from_square(Square::VARIANTS[index]));
        index += 1;
    }
    EnumMap::from_array(table)
}
