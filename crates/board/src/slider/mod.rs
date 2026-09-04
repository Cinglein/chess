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
    use crate::bitboard::Bitboard;
    use crate::square::Square;

    trait Checks: Slider {
        fn lookups_match_ray_walking_for_every_relevant_occupancy() {
            for square in Square::iter() {
                for occupied in Self::relevant_occupancy(square).subsets() {
                    assert_eq!(
                        Self::attacks(square, occupied),
                        Self::attacks_by_ray(square, occupied),
                        "{square} {occupied}"
                    );
                }
            }
        }

        fn lookups_ignore_pieces_outside_the_relevant_occupancy() {
            proptest!(|(bits: u64)| {
                let occupied = Bitboard::from_bits(bits);
                for square in Square::iter() {
                    prop_assert_eq!(Self::attacks(square, occupied), Self::attacks_by_ray(square, occupied));
                }
            });
        }
    }

    impl<S: Slider> Checks for S {}

    #[test]
    fn lookups_match_ray_walking_for_every_relevant_occupancy() {
        Rook::lookups_match_ray_walking_for_every_relevant_occupancy();
        Bishop::lookups_match_ray_walking_for_every_relevant_occupancy();
    }

    #[test]
    fn lookups_ignore_pieces_outside_the_relevant_occupancy() {
        Rook::lookups_ignore_pieces_outside_the_relevant_occupancy();
        Bishop::lookups_ignore_pieces_outside_the_relevant_occupancy();
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
