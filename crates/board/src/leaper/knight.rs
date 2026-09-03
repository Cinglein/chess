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
