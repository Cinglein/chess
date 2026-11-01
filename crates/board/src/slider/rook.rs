use super::bits::{AttackTable, ROOK};
use super::{Rays, Slider};
use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::square::Square;

pub struct Rook;

static TABLE: AttackTable<{ Rook::RAYS.table_size() }> = AttackTable::new(Rook::RAYS, &ROOK);

impl Slider for Rook {
    const RAYS: Rays = Rays::new([
        Direction::NORTH,
        Direction::EAST,
        Direction::SOUTH,
        Direction::WEST,
    ]);

    fn attacks(square: Square, occupied: Bitboard) -> Bitboard {
        TABLE.attacks(square, occupied)
    }
}
