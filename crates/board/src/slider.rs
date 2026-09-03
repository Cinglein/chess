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
    use strum::IntoEnumIterator;

    use super::{Bishop, Rook, Slider};
    use crate::bitboard::Bitboard;
    use crate::square::Square;

    fn lookups_match_ray_walking_for_every_relevant_occupancy<S: Slider>() {
        for square in Square::iter() {
            for occupied in S::relevant_occupancy(square).subsets() {
                assert_eq!(
                    S::attacks(square, occupied),
                    S::attacks_by_ray(square, occupied),
                    "{square} {occupied}"
                );
            }
        }
    }

    fn lookups_ignore_pieces_outside_the_relevant_occupancy<S: Slider>() {
        let mut state = 0x2545_F491_4F6C_DD1D_u64;
        for _ in 0..2000 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let occupied = Bitboard::from_bits(state);
            for square in Square::iter() {
                assert_eq!(
                    S::attacks(square, occupied),
                    S::attacks_by_ray(square, occupied)
                );
            }
        }
    }

    fn attacks_are_never_empty_so_zero_marks_an_unfilled_slot<S: Slider>() {
        for square in Square::iter() {
            assert!(!S::attacks_by_ray(square, Bitboard::FULL).is_empty());
        }
    }

    #[test]
    fn rook_lookups_match_ray_walking_for_every_relevant_occupancy() {
        lookups_match_ray_walking_for_every_relevant_occupancy::<Rook>();
    }

    #[test]
    fn bishop_lookups_match_ray_walking_for_every_relevant_occupancy() {
        lookups_match_ray_walking_for_every_relevant_occupancy::<Bishop>();
    }

    #[test]
    fn rook_lookups_ignore_pieces_outside_the_relevant_occupancy() {
        lookups_ignore_pieces_outside_the_relevant_occupancy::<Rook>();
    }

    #[test]
    fn bishop_lookups_ignore_pieces_outside_the_relevant_occupancy() {
        lookups_ignore_pieces_outside_the_relevant_occupancy::<Bishop>();
    }

    #[test]
    fn sliding_attacks_are_never_empty_so_zero_marks_an_unfilled_slot() {
        attacks_are_never_empty_so_zero_marks_an_unfilled_slot::<Rook>();
        attacks_are_never_empty_so_zero_marks_an_unfilled_slot::<Bishop>();
    }
}
