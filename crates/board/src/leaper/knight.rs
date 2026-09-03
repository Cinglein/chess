use super::{Leaper, Leaps};
use crate::direction::Direction;

pub struct Knight;

impl Leaper for Knight {
    const LEAPS: Leaps = Leaps::new(&[
        &[Direction::NORTH, Direction::NORTH, Direction::EAST],
        &[Direction::NORTH, Direction::NORTH, Direction::WEST],
        &[Direction::SOUTH, Direction::SOUTH, Direction::EAST],
        &[Direction::SOUTH, Direction::SOUTH, Direction::WEST],
        &[Direction::EAST, Direction::EAST, Direction::NORTH],
        &[Direction::EAST, Direction::EAST, Direction::SOUTH],
        &[Direction::WEST, Direction::WEST, Direction::NORTH],
        &[Direction::WEST, Direction::WEST, Direction::SOUTH],
    ]);
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Knight;
    use crate::bitboard::Bitboard;
    use crate::leaper::Leaper;
    use crate::orthogonal::Orthogonal;
    use crate::square::Square;

    #[test]
    fn a_knight_in_the_corner_attacks_two_squares_and_in_the_centre_eight() {
        let corner: Bitboard = [Square::B3, Square::C2].into_iter().collect();
        assert_eq!(Knight::attacks(Square::A1), corner);
        assert_eq!(Knight::attacks(Square::D4).count(), 8);
        assert!(Knight::attacks(Square::D4).contains(Square::E6));
        assert!(!Knight::attacks(Square::D4).contains(Square::D5));
    }

    #[test]
    fn every_knight_entry_agrees_with_stepping_two_then_one() {
        let north = Orthogonal::North;
        let south = Orthogonal::South;
        let east = Orthogonal::East;
        let west = Orthogonal::West;
        for square in Square::iter() {
            let expected: Bitboard = [
                square + north + north + east,
                square + north + north + west,
                square + south + south + east,
                square + south + south + west,
                square + east + east + north,
                square + east + east + south,
                square + west + west + north,
                square + west + west + south,
            ]
            .into_iter()
            .flatten()
            .collect();
            assert_eq!(Knight::attacks(square), expected, "{square}");
        }
    }
}
