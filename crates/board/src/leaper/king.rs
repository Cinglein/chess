use crate::bitboard::Bitboard;
use crate::direction::Direction;

pub(super) const fn attacks(origin: Bitboard) -> Bitboard {
    origin
        .shift(Direction::NORTH)
        .union(origin.shift(Direction::NORTH_EAST))
        .union(origin.shift(Direction::EAST))
        .union(origin.shift(Direction::SOUTH_EAST))
        .union(origin.shift(Direction::SOUTH))
        .union(origin.shift(Direction::SOUTH_WEST))
        .union(origin.shift(Direction::WEST))
        .union(origin.shift(Direction::NORTH_WEST))
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::bitboard::Bitboard;
    use crate::direction::Direction;
    use crate::leaper::Leaper;
    use crate::square::Square;

    #[test]
    fn a_king_attacks_only_adjacent_squares() {
        let corner: Bitboard = [Square::D1, Square::F1, Square::D2, Square::E2, Square::F2]
            .into_iter()
            .collect();
        assert_eq!(Leaper::King.attacks(Square::E1), corner);
        assert_eq!(Leaper::King.attacks(Square::E4).count(), 8);
    }

    #[test]
    fn every_king_entry_agrees_with_stepping_one_in_each_direction() {
        for square in Square::iter() {
            let expected: Bitboard = Direction::iter()
                .filter_map(|direction| square + direction)
                .collect();
            assert_eq!(Leaper::King.attacks(square), expected, "{square}");
        }
    }
}
