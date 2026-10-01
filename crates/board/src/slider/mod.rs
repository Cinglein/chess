mod attack_table;
mod bishop;
mod magic;
mod magics;
mod rays;
mod rook;

pub use bishop::Bishop;
pub use magic::Magic;
pub use rays::Rays;
pub use rook::Rook;

use crate::bitboard::Bitboard;
use crate::square::Square;

pub trait Slider {
    const RAYS: Rays;

    #[must_use]
    fn attacks(square: Square, occupied: Bitboard) -> Bitboard;

    #[must_use]
    fn attacks_by_ray(square: Square, occupied: Bitboard) -> Bitboard {
        Self::RAYS.attacks_by_ray(square, occupied)
    }

    #[must_use]
    fn relevant_occupancy(square: Square) -> Bitboard {
        Self::RAYS.relevant_occupancy(square)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use proptest::prelude::*;
    use strum::IntoEnumIterator;

    use super::{Bishop, Rook, Slider};
    use crate::bitboard::{Bitboard, SubsetIter};
    use crate::square::Square;

    trait Checks: Slider {
        fn first_relevant_occupancy_where_the_lookup_differs() -> Option<(Square, Bitboard)> {
            Square::iter()
                .flat_map(|square| {
                    SubsetIter::new(Self::relevant_occupancy(square))
                        .map(move |occupied| (square, occupied))
                })
                .find(|(square, occupied)| {
                    Self::attacks(*square, *occupied) != Self::attacks_by_ray(*square, *occupied)
                })
        }

        fn first_square_where_irrelevant_pieces_matter(occupied: Bitboard) -> Option<Square> {
            Square::iter().find(|square| {
                let relevant = occupied & Self::relevant_occupancy(*square);
                Self::attacks(*square, occupied) != Self::attacks_by_ray(*square, relevant)
            })
        }
    }

    impl<S: Slider> Checks for S {}

    #[test]
    fn lookups_match_ray_walking_for_every_relevant_occupancy() {
        assert_eq!(
            Rook::first_relevant_occupancy_where_the_lookup_differs(),
            None
        );
        assert_eq!(
            Bishop::first_relevant_occupancy_where_the_lookup_differs(),
            None
        );
    }

    #[test]
    fn lookups_ignore_pieces_outside_the_relevant_occupancy() {
        proptest!(|(occupied: Bitboard)| {
            prop_assert_eq!(
                (Rook::first_square_where_irrelevant_pieces_matter(occupied), Bishop::first_square_where_irrelevant_pieces_matter(occupied)),
                (None, None)
            );
        });
    }

    #[test]
    fn attacks_are_never_empty_so_zero_marks_an_unfilled_slot() {
        for (rays, square) in [Rook::RAYS, Bishop::RAYS]
            .into_iter()
            .cartesian_product(Square::iter())
        {
            assert!(
                !rays.attacks_by_ray(square, Bitboard::FULL).is_empty(),
                "{square}"
            );
        }
    }
}
