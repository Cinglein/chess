use strum::{EnumCount, VariantArray};

use crate::bitboard::Bitboard;
use crate::bitboard::SetOperation;
use crate::diagonal::Diagonal;
use crate::direction::Direction;
use crate::file::File;
use crate::orthogonal::Orthogonal;
use crate::rank::Rank;
use crate::square::Square;

const _: () = assert!(Orthogonal::COUNT == Diagonal::COUNT);

#[derive(Clone, Copy)]
pub struct Rays([Direction; Orthogonal::COUNT]);

impl Rays {
    #[must_use]
    pub const fn new(directions: [Direction; Orthogonal::COUNT]) -> Rays {
        Rays(directions)
    }

    #[must_use]
    pub const fn attacks_by_ray(self, square: Square, occupied: Bitboard) -> Bitboard {
        let origin = Bitboard::from_square(square);
        let mut attacks = Bitboard::EMPTY;
        let mut directions: &[Direction] = &self.0;
        while let [direction, rest @ ..] = directions {
            attacks = attacks.combined(
                Self::cast(origin.shift(*direction), *direction, occupied),
                SetOperation::Union,
            );
            directions = rest;
        }
        attacks
    }

    #[must_use]
    pub const fn relevant_occupancy(self, square: Square) -> Bitboard {
        let outer_ranks = Bitboard::rank(Rank::One)
            .combined(Bitboard::rank(Rank::Eight), SetOperation::Union)
            .combined(Bitboard::rank(square.rank()), SetOperation::Difference);
        let outer_files = Bitboard::file(File::A)
            .combined(Bitboard::file(File::H), SetOperation::Union)
            .combined(Bitboard::file(square.file()), SetOperation::Difference);
        self.attacks_by_ray(square, Bitboard::EMPTY).combined(
            outer_ranks.combined(outer_files, SetOperation::Union),
            SetOperation::Difference,
        )
    }

    #[must_use]
    pub const fn table_size(self) -> usize {
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
        let beyond = frontier
            .combined(occupied, SetOperation::Difference)
            .shift(direction);
        frontier.combined(Self::cast(beyond, direction, occupied), SetOperation::Union)
    }
}
