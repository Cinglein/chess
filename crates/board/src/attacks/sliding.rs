use core::sync::atomic::{AtomicU64, Ordering};

use super::magics;
use crate::bitboard::Bitboard;
use crate::square::{Direction, File, Rank, Square};

const ROOK_MAGICS: [Magic; Square::COUNT] = magics_for(Slider::Rook, &magics::ROOK);
const BISHOP_MAGICS: [Magic; Square::COUNT] = magics_for(Slider::Bishop, &magics::BISHOP);

static ROOK_TABLE: [AtomicU64; Slider::Rook.table_size()] =
    [const { AtomicU64::new(0) }; Slider::Rook.table_size()];
static BISHOP_TABLE: [AtomicU64; Slider::Bishop.table_size()] =
    [const { AtomicU64::new(0) }; Slider::Bishop.table_size()];

pub(super) fn lookup(slider: Slider, square: Square, occupied: Bitboard) -> Bitboard {
    let (magic, table): (&Magic, &[AtomicU64]) = match slider {
        Slider::Rook => (&ROOK_MAGICS[square.index()], &ROOK_TABLE),
        Slider::Bishop => (&BISHOP_MAGICS[square.index()], &BISHOP_TABLE),
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

const fn magics_for(slider: Slider, multipliers: &[u64; Square::COUNT]) -> [Magic; Square::COUNT] {
    let mut table = [Magic::new(Bitboard::EMPTY, 0, 0); Square::COUNT];
    let mut offset = 0;
    let mut index = 0;
    while index < Square::COUNT {
        let mask = slider.relevant_occupancy(Square::ALL[index]);
        let magic = Magic::new(mask, multipliers[index], offset);
        offset += magic.table_size();
        table[index] = magic;
        index += 1;
    }
    table
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Slider {
    Rook,
    Bishop,
}

impl Slider {
    pub const ALL: [Slider; 2] = [Slider::Rook, Slider::Bishop];

    #[must_use]
    pub const fn table_size(self) -> usize {
        let mut total = 0;
        let mut index = 0;
        while index < Square::COUNT {
            total += 1 << self.relevant_occupancy(Square::ALL[index]).count();
            index += 1;
        }
        total
    }

    #[must_use]
    pub const fn directions(self) -> &'static [Direction; 4] {
        match self {
            Slider::Rook => &Direction::ORTHOGONAL,
            Slider::Bishop => &Direction::DIAGONAL,
        }
    }

    #[must_use]
    pub const fn attacks_by_ray(self, square: Square, occupied: Bitboard) -> Bitboard {
        let directions = self.directions();
        let mut attacks = Bitboard::EMPTY;
        let mut index = 0;
        while index < directions.len() {
            attacks = attacks.union(ray(square, directions[index], occupied));
            index += 1;
        }
        attacks
    }

    #[must_use]
    pub const fn relevant_occupancy(self, square: Square) -> Bitboard {
        let outer_ranks = Bitboard::rank(Rank::One)
            .union(Bitboard::rank(Rank::Eight))
            .difference(Bitboard::rank(square.rank()));
        let outer_files = Bitboard::file(File::A)
            .union(Bitboard::file(File::H))
            .difference(Bitboard::file(square.file()));
        self.attacks_by_ray(square, Bitboard::EMPTY)
            .difference(outer_ranks.union(outer_files))
    }
}

const fn ray(square: Square, direction: Direction, occupied: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let mut current = square;
    while let Some(next) = current.offset(direction) {
        attacks = attacks.with(next);
        if occupied.contains(next) {
            break;
        }
        current = next;
    }
    attacks
}

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
    pub const fn multiplier(&self) -> u64 {
        self.multiplier
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
        self.offset + (hash >> self.shift) as usize
    }
}
