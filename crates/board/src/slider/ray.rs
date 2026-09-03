use crate::bitboard::Bitboard;
use crate::direction::Direction;

pub(super) const fn cast(origin: Bitboard, direction: Direction, occupied: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let mut frontier = origin.shift(direction);
    while !frontier.is_empty() {
        attacks = attacks.union(frontier);
        frontier = frontier.difference(occupied).shift(direction);
    }
    attacks
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::cast;
    use crate::bitboard::Bitboard;
    use crate::direction::Direction;
    use crate::square::Square;

    #[test]
    fn a_ray_stops_at_the_first_occupied_square_and_includes_it() {
        let origin = Bitboard::from_square(Square::D4);
        let blocker = Bitboard::from_square(Square::D6);
        let expected: Bitboard = [Square::D5, Square::D6].into_iter().collect();
        assert_eq!(cast(origin, Direction::NORTH, blocker), expected);
    }

    #[test]
    fn every_ray_on_an_empty_board_agrees_with_stepping_to_the_edge() {
        for square in Square::iter() {
            for direction in Direction::iter() {
                let mut expected = Bitboard::EMPTY;
                let mut current = square + direction;
                while let Some(next) = current {
                    expected = expected.with(next);
                    current = next + direction;
                }
                let origin = Bitboard::from_square(square);
                assert_eq!(
                    cast(origin, direction, Bitboard::EMPTY),
                    expected,
                    "{square} {direction:?}"
                );
            }
        }
    }
}
