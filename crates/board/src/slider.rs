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
    use core::iter;

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
            iter::successors(Some(0x2545_F491_4F6C_DD1D_u64), |&state| {
                let shaken = state ^ (state << 13);
                let stirred = shaken ^ (shaken >> 7);
                Some(stirred ^ (stirred << 17))
            })
            .take(2000)
            .map(Bitboard::from_bits)
            .flat_map(|occupied| Square::iter().map(move |square| (square, occupied)))
            .for_each(|(square, occupied)| {
                assert_eq!(
                    Self::attacks(square, occupied),
                    Self::attacks_by_ray(square, occupied)
                );
            });
        }

        fn attacks_are_never_empty_so_zero_marks_an_unfilled_slot() {
            for square in Square::iter() {
                assert!(!Self::attacks_by_ray(square, Bitboard::FULL).is_empty());
            }
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
        Rook::attacks_are_never_empty_so_zero_marks_an_unfilled_slot();
        Bishop::attacks_are_never_empty_so_zero_marks_an_unfilled_slot();
    }
}
