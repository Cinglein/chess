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

#[cfg(test)]
mod tests {
    use super::Bishop;
    use crate::bitboard::Bitboard;
    use crate::slider::Slider;
    use crate::square::Square;

    #[test]
    fn a_bishop_stops_at_the_first_blocker_and_includes_it() {
        let blockers: Bitboard = [Square::F6, Square::B2].into_iter().collect();
        let expected: Bitboard = [
            Square::E5,
            Square::F6,
            Square::C3,
            Square::B2,
            Square::E3,
            Square::F2,
            Square::G1,
            Square::C5,
            Square::B6,
            Square::A7,
        ]
        .into_iter()
        .collect();
        assert_eq!(Bishop::attacks(Square::D4, blockers), expected);
    }

    #[test]
    fn relevant_occupancy_ignores_board_edges() {
        assert_eq!(Bishop::relevant_occupancy(Square::A1).count(), 6);
        assert_eq!(Bishop::relevant_occupancy(Square::D4).count(), 9);
    }
}
