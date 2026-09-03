use super::{Leaper, Leaps};
use crate::direction::Direction;

pub struct King;

impl Leaper for King {
    const LEAPS: Leaps = Leaps::new(&[
        &[Direction::NORTH],
        &[Direction::NORTH_EAST],
        &[Direction::EAST],
        &[Direction::SOUTH_EAST],
        &[Direction::SOUTH],
        &[Direction::SOUTH_WEST],
        &[Direction::WEST],
        &[Direction::NORTH_WEST],
    ]);
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::King;
    use crate::bitboard::Bitboard;
    use crate::direction::Direction;
    use crate::leaper::Leaper;
    use crate::square::Square;

    #[test]
    fn a_king_attacks_only_adjacent_squares() {
        let corner: Bitboard = [Square::D1, Square::F1, Square::D2, Square::E2, Square::F2]
            .into_iter()
            .collect();
        assert_eq!(King::attacks(Square::E1), corner);
        assert_eq!(King::attacks(Square::E4).count(), 8);
    }

    #[test]
    fn every_king_entry_agrees_with_stepping_one_in_each_direction() {
        for square in Square::iter() {
            let expected: Bitboard = Direction::iter()
                .filter_map(|direction| square + direction)
                .collect();
            assert_eq!(King::attacks(square), expected, "{square}");
        }
    }
}
