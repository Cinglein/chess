use core::sync::atomic::{AtomicU64, Ordering};

use enum_map::EnumMap;
use strum::{EnumCount, VariantArray};

use super::{Magic, Slider, magics};
use crate::bitboard::Bitboard;
use crate::square::Square;

const ROOK_MAGICS: EnumMap<Square, Magic> = magics_for(Slider::Rook, &magics::ROOK);
const BISHOP_MAGICS: EnumMap<Square, Magic> = magics_for(Slider::Bishop, &magics::BISHOP);

static ROOK_TABLE: [AtomicU64; Slider::Rook.table_size()] =
    [const { AtomicU64::new(0) }; Slider::Rook.table_size()];
static BISHOP_TABLE: [AtomicU64; Slider::Bishop.table_size()] =
    [const { AtomicU64::new(0) }; Slider::Bishop.table_size()];

pub(super) fn lookup(slider: Slider, square: Square, occupied: Bitboard) -> Bitboard {
    let (magic, table): (&Magic, &[AtomicU64]) = match slider {
        Slider::Rook => (&ROOK_MAGICS[square], &ROOK_TABLE),
        Slider::Bishop => (&BISHOP_MAGICS[square], &BISHOP_TABLE),
    };
    let slot = &table[magic.index(occupied)];
    let cached = slot.load(Ordering::Relaxed);
    if cached != 0 {
        return Bitboard::from_bits(cached);
    }
    let attacks = slider.attacks_by_ray(square, occupied);
    slot.store(attacks.bits(), Ordering::Relaxed);
    attacks
}

const fn magics_for(slider: Slider, multipliers: &EnumMap<Square, u64>) -> EnumMap<Square, Magic> {
    let mut magics = [Magic::new(Bitboard::EMPTY, 0, 0); Square::COUNT];
    let mut offset = 0;
    let mut index = 0;
    while index < Square::COUNT {
        let square = Square::VARIANTS[index];
        let magic = Magic::new(
            slider.relevant_occupancy(square),
            multipliers.as_array()[index],
            offset,
        );
        offset += magic.table_size();
        magics[index] = magic;
        index += 1;
    }
    EnumMap::from_array(magics)
}
