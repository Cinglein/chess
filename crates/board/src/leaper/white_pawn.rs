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
