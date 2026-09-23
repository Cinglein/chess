use strum::VariantArray;

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
        let mut directions: &[Direction] = &self.0;
        while let [direction, rest @ ..] = directions {
            attacks = attacks.union(Self::cast(origin.shift(*direction), *direction, occupied));
            directions = rest;
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
        let mut squares = Square::VARIANTS;
        while let [square, rest @ ..] = squares {
            total += 1 << self.relevant_occupancy(*square).count();
            squares = rest;
        }
        total
    }

    const fn cast(frontier: Bitboard, direction: Direction, occupied: Bitboard) -> Bitboard {
        if frontier.is_empty() {
            return Bitboard::EMPTY;
        }
        let beyond = frontier.difference(occupied).shift(direction);
        frontier.union(Self::cast(beyond, direction, occupied))
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Rays;
    use crate::bitboard::Bitboard;
    use crate::direction::Direction;
    use crate::slider::{Bishop, Rook, Slider};
    use crate::square::Square;

    const RELEVANT_SQUARES: [(Square, u32, u32); 2] = [(Square::A1, 12, 6), (Square::D4, 10, 9)];

    #[test]
    fn relevant_occupancy_drops_the_edge_squares() {
        for (square, rook, bishop) in RELEVANT_SQUARES {
            assert_eq!(
                Rook::RAYS.relevant_occupancy(square).count(),
                rook,
                "{square}"
            );
            assert_eq!(
                Bishop::RAYS.relevant_occupancy(square).count(),
                bishop,
                "{square}"
            );
        }
    }

    #[test]
    fn every_ray_agrees_with_stepping_until_the_first_blocker() {
        for square in Square::iter() {
            for direction in Direction::iter() {
                let blockers = Bitboard::rank(square.rank()) ^ Bitboard::file(square.file());
                let mut expected = Bitboard::EMPTY;
                let mut current = square + direction;
                while let Some(next) = current {
                    expected = expected.with(next);
                    current = (!blockers.contains(next))
                        .then(|| next + direction)
                        .flatten();
                }
                let frontier = Bitboard::from_square(square).shift(direction);
                assert_eq!(
                    Rays::cast(frontier, direction, blockers),
                    expected,
                    "{square} {direction:?}"
                );
            }
        }
    }
}
