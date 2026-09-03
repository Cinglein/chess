use crate::bitboard::Bitboard;
use crate::direction::Direction;

pub(super) const fn attacks(origin: Bitboard) -> Bitboard {
    let north = origin.shift(Direction::NORTH).shift(Direction::NORTH);
    let south = origin.shift(Direction::SOUTH).shift(Direction::SOUTH);
    let east = origin.shift(Direction::EAST).shift(Direction::EAST);
    let west = origin.shift(Direction::WEST).shift(Direction::WEST);
    north
        .shift(Direction::EAST)
        .union(north.shift(Direction::WEST))
        .union(south.shift(Direction::EAST))
        .union(south.shift(Direction::WEST))
        .union(east.shift(Direction::NORTH))
        .union(east.shift(Direction::SOUTH))
        .union(west.shift(Direction::NORTH))
        .union(west.shift(Direction::SOUTH))
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::bitboard::Bitboard;
    use crate::leaper::Leaper;
    use crate::orthogonal::Orthogonal;
    use crate::square::Square;

    #[test]
    fn a_knight_in_the_corner_attacks_two_squares_and_in_the_centre_eight() {
        let corner: Bitboard = [Square::B3, Square::C2].into_iter().collect();
        assert_eq!(Leaper::Knight.attacks(Square::A1), corner);
        assert_eq!(Leaper::Knight.attacks(Square::D4).count(), 8);
        assert!(Leaper::Knight.attacks(Square::D4).contains(Square::E6));
        assert!(!Leaper::Knight.attacks(Square::D4).contains(Square::D5));
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
            assert_eq!(Leaper::Knight.attacks(square), expected, "{square}");
        }
    }
}
