use super::{Leaper, Leaps, Pawn};
use crate::color::Color;
use crate::direction::Direction;
use crate::orthogonal::Orthogonal;
use crate::rank::Rank;

pub struct WhitePawn;

impl Leaper for WhitePawn {
    const LEAPS: Leaps = Leaps::new(&[&[Direction::NORTH_EAST], &[Direction::NORTH_WEST]]);
}

impl Pawn for WhitePawn {
    const COLOR: Color = Color::White;
    const PUSH: Orthogonal = Orthogonal::North;
    const START_RANK: Rank = Rank::Two;
    const PROMOTION_RANK: Rank = Rank::Eight;
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
