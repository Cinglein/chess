use super::attack_table::AttackTable;
use super::{Rays, Slider, magics};
use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::square::Square;

pub struct Bishop;

static TABLE: AttackTable<{ Bishop::RAYS.table_size() }> =
    AttackTable::new(Bishop::RAYS, &magics::BISHOP);

impl Slider for Bishop {
    const RAYS: Rays = Rays::new([
        Direction::NORTH_EAST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
        Direction::NORTH_WEST,
    ]);

    fn attacks(square: Square, occupied: Bitboard) -> Bitboard {
        TABLE.attacks(square, occupied)
    }
}
