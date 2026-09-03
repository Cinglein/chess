use super::{Leaper, Leaps};
use crate::direction::Direction;

pub struct BlackPawn;

impl Leaper for BlackPawn {
    const LEAPS: Leaps = Leaps::new(&[&[Direction::SOUTH_EAST], &[Direction::SOUTH_WEST]]);
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::BlackPawn;
    use crate::bitboard::Bitboard;
    use crate::diagonal::Diagonal;
    use crate::leaper::Leaper;
    use crate::square::Square;

    #[test]
    fn black_pawns_attack_diagonally_down_the_board() {
        let expected: Bitboard = [Square::D3, Square::F3].into_iter().collect();
        assert_eq!(BlackPawn::attacks(Square::E4), expected);
        assert_eq!(BlackPawn::attacks(Square::A1), Bitboard::EMPTY);
    }

    #[test]
    fn every_black_pawn_entry_agrees_with_stepping_diagonally_south() {
        for square in Square::iter() {
            let expected: Bitboard = [square + Diagonal::SouthEast, square + Diagonal::SouthWest]
                .into_iter()
                .flatten()
                .collect();
            assert_eq!(BlackPawn::attacks(square), expected, "{square}");
        }
    }
}
