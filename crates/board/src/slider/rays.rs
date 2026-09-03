use strum::{EnumCount, VariantArray};

use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Clone, Copy)]
pub struct Rays([Direction; 4]);

impl Rays {
    #[must_use]
    pub const fn new(directions: [Direction; 4]) -> Rays {
        Rays(directions)
    }

    #[must_use]
    pub const fn attacks_by_ray(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let origin = Bitboard::from_square(square);
        let mut attacks = Bitboard::EMPTY;
        let mut index = 0;
        while index < self.0.len() {
            attacks = attacks.union(Self::cast(origin, self.0[index], occupied));
            index += 1;
        }
        attacks
    }

    #[must_use]
    pub const fn relevant_occupancy(&self, square: Square) -> Bitboard {
        let outer_ranks = Bitboard::rank(Rank::One)
            .union(Bitboard::rank(Rank::Eight))
            .difference(Bitboard::rank(square.rank()));
        let outer_files = Bitboard::file(File::A)
            .union(Bitboard::file(File::H))
            .difference(Bitboard::file(square.file()));
        self.attacks_by_ray(square, Bitboard::EMPTY)
            .difference(outer_ranks.union(outer_files))
    }

    #[must_use]
    pub const fn table_size(&self) -> usize {
        let mut total = 0;
        let mut index = 0;
        while index < Square::COUNT {
            total += 1 << self.relevant_occupancy(Square::VARIANTS[index]).count();
            index += 1;
        }
        total
    }

    const fn cast(origin: Bitboard, direction: Direction, occupied: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let mut frontier = origin.shift(direction);
        while !frontier.is_empty() {
            attacks = attacks.union(frontier);
            frontier = frontier.difference(occupied).shift(direction);
        }
        attacks
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Rays;
    use crate::bitboard::Bitboard;
    use crate::direction::Direction;
    use crate::square::Square;

    #[test]
    fn a_ray_stops_at_the_first_occupied_square_and_includes_it() {
        let origin = Bitboard::from_square(Square::D4);
        let blocker = Bitboard::from_square(Square::D6);
        let expected: Bitboard = [Square::D5, Square::D6].into_iter().collect();
        assert_eq!(Rays::cast(origin, Direction::NORTH, blocker), expected);
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
                    Rays::cast(origin, direction, Bitboard::EMPTY),
                    expected,
                    "{square} {direction:?}"
                );
            }
        }
    }
}
