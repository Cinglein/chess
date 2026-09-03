mod magic;
mod magics;
mod ray;
mod table;

pub use magic::Magic;

use enum_map::Enum;
use strum::{EnumCount, EnumIter, FromRepr, VariantArray};

use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Enum, EnumCount, EnumIter, FromRepr, VariantArray,
)]
#[repr(u8)]
pub enum Slider {
    Rook,
    Bishop,
}

impl Slider {
    #[must_use]
    pub fn attacks(self, square: Square, occupied: Bitboard) -> Bitboard {
        table::lookup(self, square, occupied)
    }

    #[must_use]
    pub const fn attacks_by_ray(self, square: Square, occupied: Bitboard) -> Bitboard {
        let origin = Bitboard::from_square(square);
        let directions = self.directions();
        let mut attacks = Bitboard::EMPTY;
        let mut index = 0;
        while index < directions.len() {
            attacks = attacks.union(ray::cast(origin, directions[index], occupied));
            index += 1;
        }
        attacks
    }

    #[must_use]
    pub const fn relevant_occupancy(self, square: Square) -> Bitboard {
        let outer_ranks = Bitboard::rank(Rank::One)
            .union(Bitboard::rank(Rank::Eight))
            .difference(Bitboard::rank(square.rank()));
        let outer_files = Bitboard::file(File::A)
            .union(Bitboard::file(File::H))
            .difference(Bitboard::file(square.file()));
        self.attacks_by_ray(square, Bitboard::EMPTY)
            .difference(outer_ranks.union(outer_files))
    }

    const fn directions(self) -> [Direction; 4] {
        match self {
            Slider::Rook => [
                Direction::NORTH,
                Direction::EAST,
                Direction::SOUTH,
                Direction::WEST,
            ],
            Slider::Bishop => [
                Direction::NORTH_EAST,
                Direction::SOUTH_EAST,
                Direction::SOUTH_WEST,
                Direction::NORTH_WEST,
            ],
        }
    }

    const fn table_size(self) -> usize {
        let mut total = 0;
        let mut index = 0;
        while index < Square::COUNT {
            total += 1 << self.relevant_occupancy(Square::VARIANTS[index]).count();
            index += 1;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Slider;
    use crate::bitboard::Bitboard;
    use crate::square::Square;

    #[test]
    fn a_rook_on_an_empty_board_attacks_fourteen_squares() {
        for square in Square::iter() {
            assert_eq!(
                Slider::Rook.attacks(square, Bitboard::EMPTY).count(),
                14,
                "{square}"
            );
        }
    }

    #[test]
    fn a_bishop_stops_at_the_first_blocker_and_includes_it() {
        let blockers: Bitboard = [Square::F6, Square::B2].into_iter().collect();
        let expected: Bitboard = [
            Square::E5,
            Square::F6,
            Square::C3,
            Square::B2,
            Square::E3,
            Square::F2,
            Square::G1,
            Square::C5,
            Square::B6,
            Square::A7,
        ]
        .into_iter()
        .collect();
        assert_eq!(Slider::Bishop.attacks(Square::D4, blockers), expected);
    }

    #[test]
    fn relevant_occupancy_ignores_board_edges() {
        assert_eq!(Slider::Rook.relevant_occupancy(Square::A1).count(), 12);
        assert_eq!(Slider::Rook.relevant_occupancy(Square::D4).count(), 10);
        assert_eq!(Slider::Bishop.relevant_occupancy(Square::A1).count(), 6);
        assert_eq!(Slider::Bishop.relevant_occupancy(Square::D4).count(), 9);
        assert!(
            !Slider::Rook
                .relevant_occupancy(Square::D4)
                .contains(Square::D8)
        );
    }

    #[test]
    fn sliding_attacks_are_never_empty_so_zero_marks_an_unfilled_slot() {
        for slider in Slider::iter() {
            for square in Square::iter() {
                assert!(!slider.attacks_by_ray(square, Bitboard::FULL).is_empty());
            }
        }
    }

    #[test]
    fn magic_lookups_match_ray_walking_for_every_relevant_occupancy() {
        for slider in Slider::iter() {
            for square in Square::iter() {
                for occupied in slider.relevant_occupancy(square).subsets() {
                    assert_eq!(
                        slider.attacks(square, occupied),
                        slider.attacks_by_ray(square, occupied),
                        "{slider:?} {square} {occupied}"
                    );
                }
            }
        }
    }

    #[test]
    fn magic_lookups_ignore_pieces_outside_the_relevant_occupancy() {
        let mut state = 0x2545_F491_4F6C_DD1D_u64;
        for _ in 0..2000 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let occupied = Bitboard::from_bits(state);
            for slider in Slider::iter() {
                for square in Square::iter() {
                    assert_eq!(
                        slider.attacks(square, occupied),
                        slider.attacks_by_ray(square, occupied)
                    );
                }
            }
        }
    }
}
