use super::attack_table::AttackTable;
use super::{Rays, Slider, magics};
use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::square::Square;

pub struct Rook;

static TABLE: AttackTable<{ Rook::RAYS.table_size() }> =
    AttackTable::new(Rook::RAYS, &magics::ROOK);

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

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Rook;
    use crate::bitboard::Bitboard;
    use crate::slider::Slider;
    use crate::square::Square;

    #[test]
    fn a_rook_on_an_empty_board_attacks_fourteen_squares() {
        for square in Square::iter() {
            assert_eq!(
                Rook::attacks(square, Bitboard::EMPTY).count(),
                14,
                "{square}"
            );
        }
    }

    #[test]
    fn relevant_occupancy_ignores_board_edges() {
        assert_eq!(Rook::relevant_occupancy(Square::A1).count(), 12);
        assert_eq!(Rook::relevant_occupancy(Square::D4).count(), 10);
        assert!(!Rook::relevant_occupancy(Square::D4).contains(Square::D8));
    }
}
