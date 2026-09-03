use super::{Leaper, Leaps};
use crate::direction::Direction;

pub struct WhitePawn;

impl Leaper for WhitePawn {
    const LEAPS: Leaps = Leaps::new(&[&[Direction::NORTH_EAST], &[Direction::NORTH_WEST]]);
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::WhitePawn;
    use crate::bitboard::Bitboard;
    use crate::diagonal::Diagonal;
    use crate::leaper::Leaper;
    use crate::square::Square;

    #[test]
    fn white_pawns_attack_diagonally_up_the_board() {
        let expected: Bitboard = [Square::D5, Square::F5].into_iter().collect();
        assert_eq!(WhitePawn::attacks(Square::E4), expected);
        assert_eq!(
            WhitePawn::attacks(Square::A2),
            Bitboard::from_square(Square::B3)
        );
        assert_eq!(WhitePawn::attacks(Square::H8), Bitboard::EMPTY);
    }

    #[test]
    fn every_white_pawn_entry_agrees_with_stepping_diagonally_north() {
        for square in Square::iter() {
            let expected: Bitboard = [square + Diagonal::NorthEast, square + Diagonal::NorthWest]
                .into_iter()
                .flatten()
                .collect();
            assert_eq!(WhitePawn::attacks(square), expected, "{square}");
        }
    }
}
