use super::{Leaper, Leaps, Pawn};
use crate::color::Color;
use crate::direction::Direction;
use crate::orthogonal::Orthogonal;
use crate::rank::Rank;

pub struct BlackPawn;

impl Leaper for BlackPawn {
    const LEAPS: Leaps = Leaps::new(&[&[Direction::SOUTH_EAST], &[Direction::SOUTH_WEST]]);
}

impl Pawn for BlackPawn {
    const COLOR: Color = Color::Black;
    const PUSH: Orthogonal = Orthogonal::South;
    const START_RANK: Rank = Rank::Seven;
    const PROMOTION_RANK: Rank = Rank::One;
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
