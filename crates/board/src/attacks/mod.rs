mod leapers;
mod magics;
mod sliding;

use crate::bitboard::Bitboard;
use crate::piece::Color;
use crate::square::Square;

pub use sliding::{Magic, Slider};

#[must_use]
pub fn rook(square: Square, occupied: Bitboard) -> Bitboard {
    sliding::lookup(Slider::Rook, square, occupied)
}

#[must_use]
pub fn bishop(square: Square, occupied: Bitboard) -> Bitboard {
    sliding::lookup(Slider::Bishop, square, occupied)
}

#[must_use]
pub fn queen(square: Square, occupied: Bitboard) -> Bitboard {
    rook(square, occupied) | bishop(square, occupied)
}

#[must_use]
pub fn knight(square: Square) -> Bitboard {
    leapers::KNIGHT[square.index()]
}

#[must_use]
pub fn king(square: Square) -> Bitboard {
    leapers::KING[square.index()]
}

#[must_use]
pub fn pawn(color: Color, square: Square) -> Bitboard {
    leapers::PAWN[color.index()][square.index()]
}

#[cfg(test)]
mod tests {
    use super::{Slider, bishop, king, knight, pawn, queen, rook};
    use crate::bitboard::Bitboard;
    use crate::piece::Color;
    use crate::square::Square;

    fn squares(list: &[Square]) -> Bitboard {
        list.iter().copied().collect()
    }

    #[test]
    fn a_knight_in_the_corner_attacks_two_squares_and_in_the_centre_eight() {
        assert_eq!(knight(Square::A1), squares(&[Square::B3, Square::C2]));
        assert_eq!(knight(Square::D4).count(), 8);
        assert!(knight(Square::D4).contains(Square::E6));
        assert!(!knight(Square::D4).contains(Square::D5));
    }

    #[test]
    fn a_king_attacks_only_adjacent_squares() {
        assert_eq!(
            king(Square::E1),
            squares(&[Square::D1, Square::F1, Square::D2, Square::E2, Square::F2])
        );
        assert_eq!(king(Square::E4).count(), 8);
    }

    #[test]
    fn pawns_attack_diagonally_forward_for_their_own_colour() {
        assert_eq!(
            pawn(Color::White, Square::E4),
            squares(&[Square::D5, Square::F5])
        );
        assert_eq!(
            pawn(Color::Black, Square::E4),
            squares(&[Square::D3, Square::F3])
        );
        assert_eq!(pawn(Color::White, Square::A2), squares(&[Square::B3]));
        assert_eq!(pawn(Color::White, Square::H8), Bitboard::EMPTY);
    }

    #[test]
    fn a_rook_on_an_empty_board_attacks_fourteen_squares() {
        for square in Square::ALL {
            assert_eq!(rook(square, Bitboard::EMPTY).count(), 14, "{square}");
        }
    }

    #[test]
    fn a_bishop_stops_at_the_first_blocker_and_includes_it() {
        let blockers = squares(&[Square::F6, Square::B2]);
        assert_eq!(
            bishop(Square::D4, blockers),
            squares(&[
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
            ])
        );
    }

    #[test]
    fn a_queen_attacks_the_union_of_rook_and_bishop_lines() {
        let occupied = squares(&[Square::D6, Square::G4, Square::B2]);
        assert_eq!(
            queen(Square::D4, occupied),
            rook(Square::D4, occupied) | bishop(Square::D4, occupied)
        );
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
        for slider in Slider::ALL {
            for square in Square::ALL {
                assert!(!slider.attacks_by_ray(square, Bitboard::FULL).is_empty());
            }
        }
    }

    #[test]
    fn magic_lookups_match_ray_walking_for_every_relevant_occupancy() {
        for slider in Slider::ALL {
            for square in Square::ALL {
                for occupied in slider.relevant_occupancy(square).subsets() {
                    let expected = slider.attacks_by_ray(square, occupied);
                    let actual = match slider {
                        Slider::Rook => rook(square, occupied),
                        Slider::Bishop => bishop(square, occupied),
                    };
                    assert_eq!(actual, expected, "{slider:?} {square}\n{occupied}");
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
            for square in Square::ALL {
                assert_eq!(
                    rook(square, occupied),
                    Slider::Rook.attacks_by_ray(square, occupied)
                );
                assert_eq!(
                    bishop(square, occupied),
                    Slider::Bishop.attacks_by_ray(square, occupied)
                );
            }
        }
    }
}
