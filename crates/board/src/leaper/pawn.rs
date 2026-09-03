use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::direction::Direction;

pub(super) const fn attacks(color: Color, origin: Bitboard) -> Bitboard {
    match color {
        Color::White => origin
            .shift(Direction::NORTH_EAST)
            .union(origin.shift(Direction::NORTH_WEST)),
        Color::Black => origin
            .shift(Direction::SOUTH_EAST)
            .union(origin.shift(Direction::SOUTH_WEST)),
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::bitboard::Bitboard;
    use crate::color::Color;
    use crate::diagonal::Diagonal;
    use crate::leaper::Leaper;
    use crate::square::Square;

    #[test]
    fn pawns_attack_diagonally_forward_for_their_own_colour() {
        let white: Bitboard = [Square::D5, Square::F5].into_iter().collect();
        let black: Bitboard = [Square::D3, Square::F3].into_iter().collect();
        assert_eq!(Leaper::pawn(Color::White).attacks(Square::E4), white);
        assert_eq!(Leaper::pawn(Color::Black).attacks(Square::E4), black);
        assert_eq!(
            Leaper::WhitePawn.attacks(Square::A2),
            Bitboard::from_square(Square::B3)
        );
        assert_eq!(Leaper::WhitePawn.attacks(Square::H8), Bitboard::EMPTY);
    }

    #[test]
    fn every_pawn_entry_agrees_with_stepping_diagonally_forward() {
        for square in Square::iter() {
            let white: Bitboard = [square + Diagonal::NorthEast, square + Diagonal::NorthWest]
                .into_iter()
                .flatten()
                .collect();
            let black: Bitboard = [square + Diagonal::SouthEast, square + Diagonal::SouthWest]
                .into_iter()
                .flatten()
                .collect();
            assert_eq!(Leaper::WhitePawn.attacks(square), white, "white {square}");
            assert_eq!(Leaper::BlackPawn.attacks(square), black, "black {square}");
        }
    }
}
