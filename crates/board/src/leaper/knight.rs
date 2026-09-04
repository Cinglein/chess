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

    const JUMPS: [[Orthogonal; 3]; 8] = [
        [Orthogonal::North, Orthogonal::North, Orthogonal::East],
        [Orthogonal::North, Orthogonal::North, Orthogonal::West],
        [Orthogonal::South, Orthogonal::South, Orthogonal::East],
        [Orthogonal::South, Orthogonal::South, Orthogonal::West],
        [Orthogonal::East, Orthogonal::East, Orthogonal::North],
        [Orthogonal::East, Orthogonal::East, Orthogonal::South],
        [Orthogonal::West, Orthogonal::West, Orthogonal::North],
        [Orthogonal::West, Orthogonal::West, Orthogonal::South],
    ];

    #[test]
    fn every_knight_entry_agrees_with_stepping_two_then_one() {
        for square in Square::iter() {
            let expected: Bitboard = JUMPS
                .iter()
                .filter_map(|steps| {
                    steps
                        .iter()
                        .try_fold(square, |current, &step| current + step)
                })
                .collect();
            assert_eq!(Knight::attacks(square), expected, "{square}");
        }
    }
}
