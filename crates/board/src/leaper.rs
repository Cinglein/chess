use enum_map::{Enum, EnumMap};
use strum::{EnumCount, EnumIter, FromRepr, VariantArray};

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::diagonal::Diagonal;
use crate::direction::Direction;
use crate::orthogonal::Orthogonal;
use crate::square::Square;

const NORTH: Direction = Direction::Orthogonal(Orthogonal::North);
const EAST: Direction = Direction::Orthogonal(Orthogonal::East);
const SOUTH: Direction = Direction::Orthogonal(Orthogonal::South);
const WEST: Direction = Direction::Orthogonal(Orthogonal::West);
const NORTH_EAST: Direction = Direction::Diagonal(Diagonal::NorthEast);
const SOUTH_EAST: Direction = Direction::Diagonal(Diagonal::SouthEast);
const SOUTH_WEST: Direction = Direction::Diagonal(Diagonal::SouthWest);
const NORTH_WEST: Direction = Direction::Diagonal(Diagonal::NorthWest);

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Enum, EnumCount, EnumIter, FromRepr, VariantArray,
)]
#[repr(u8)]
pub enum Leaper {
    Knight,
    King,
    WhitePawn,
    BlackPawn,
}

static ATTACKS: EnumMap<Leaper, EnumMap<Square, Bitboard>> = EnumMap::from_array([
    attack_table(Leaper::Knight),
    attack_table(Leaper::King),
    attack_table(Leaper::WhitePawn),
    attack_table(Leaper::BlackPawn),
]);

impl Leaper {
    #[must_use]
    pub const fn pawn(color: Color) -> Leaper {
        match color {
            Color::White => Leaper::WhitePawn,
            Color::Black => Leaper::BlackPawn,
        }
    }

    #[must_use]
    pub fn attacks(self, square: Square) -> Bitboard {
        ATTACKS[self][square]
    }

    const fn attacks_from(self, origin: Bitboard) -> Bitboard {
        match self {
            Leaper::Knight => knight_attacks(origin),
            Leaper::King => king_attacks(origin),
            Leaper::WhitePawn => origin.shift(NORTH_EAST).union(origin.shift(NORTH_WEST)),
            Leaper::BlackPawn => origin.shift(SOUTH_EAST).union(origin.shift(SOUTH_WEST)),
        }
    }
}

const fn attack_table(leaper: Leaper) -> EnumMap<Square, Bitboard> {
    let mut table = [Bitboard::EMPTY; Square::COUNT];
    let mut index = 0;
    while index < Square::COUNT {
        table[index] = leaper.attacks_from(Bitboard::from_square(Square::VARIANTS[index]));
        index += 1;
    }
    EnumMap::from_array(table)
}

const fn knight_attacks(origin: Bitboard) -> Bitboard {
    let north = origin.shift(NORTH).shift(NORTH);
    let south = origin.shift(SOUTH).shift(SOUTH);
    let east = origin.shift(EAST).shift(EAST);
    let west = origin.shift(WEST).shift(WEST);
    north
        .shift(EAST)
        .union(north.shift(WEST))
        .union(south.shift(EAST))
        .union(south.shift(WEST))
        .union(east.shift(NORTH))
        .union(east.shift(SOUTH))
        .union(west.shift(NORTH))
        .union(west.shift(SOUTH))
}

const fn king_attacks(origin: Bitboard) -> Bitboard {
    origin
        .shift(NORTH)
        .union(origin.shift(NORTH_EAST))
        .union(origin.shift(EAST))
        .union(origin.shift(SOUTH_EAST))
        .union(origin.shift(SOUTH))
        .union(origin.shift(SOUTH_WEST))
        .union(origin.shift(WEST))
        .union(origin.shift(NORTH_WEST))
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Leaper;
    use crate::bitboard::Bitboard;
    use crate::color::Color;
    use crate::diagonal::Diagonal;
    use crate::direction::Direction;
    use crate::orthogonal::Orthogonal;
    use crate::square::Square;

    fn squares(list: &[Square]) -> Bitboard {
        list.iter().copied().collect()
    }

    #[test]
    fn a_knight_in_the_corner_attacks_two_squares_and_in_the_centre_eight() {
        assert_eq!(
            Leaper::Knight.attacks(Square::A1),
            squares(&[Square::B3, Square::C2])
        );
        assert_eq!(Leaper::Knight.attacks(Square::D4).count(), 8);
        assert!(Leaper::Knight.attacks(Square::D4).contains(Square::E6));
        assert!(!Leaper::Knight.attacks(Square::D4).contains(Square::D5));
    }

    #[test]
    fn a_king_attacks_only_adjacent_squares() {
        assert_eq!(
            Leaper::King.attacks(Square::E1),
            squares(&[Square::D1, Square::F1, Square::D2, Square::E2, Square::F2])
        );
        assert_eq!(Leaper::King.attacks(Square::E4).count(), 8);
    }

    #[test]
    fn pawns_attack_diagonally_forward_for_their_own_colour() {
        assert_eq!(
            Leaper::pawn(Color::White).attacks(Square::E4),
            squares(&[Square::D5, Square::F5])
        );
        assert_eq!(
            Leaper::pawn(Color::Black).attacks(Square::E4),
            squares(&[Square::D3, Square::F3])
        );
        assert_eq!(
            Leaper::WhitePawn.attacks(Square::A2),
            squares(&[Square::B3])
        );
        assert_eq!(Leaper::WhitePawn.attacks(Square::H8), Bitboard::EMPTY);
    }

    #[test]
    fn every_table_entry_agrees_with_stepping_square_by_square() {
        let north = Orthogonal::North;
        let south = Orthogonal::South;
        let east = Orthogonal::East;
        let west = Orthogonal::West;
        for square in Square::iter() {
            let knight: Bitboard = [
                square + north + north + east,
                square + north + north + west,
                square + south + south + east,
                square + south + south + west,
                square + east + east + north,
                square + east + east + south,
                square + west + west + north,
                square + west + west + south,
            ]
            .into_iter()
            .flatten()
            .collect();
            assert_eq!(Leaper::Knight.attacks(square), knight, "knight {square}");

            let king: Bitboard = Direction::iter()
                .filter_map(|direction| square + direction)
                .collect();
            assert_eq!(Leaper::King.attacks(square), king, "king {square}");

            let white: Bitboard = [square + Diagonal::NorthEast, square + Diagonal::NorthWest]
                .into_iter()
                .flatten()
                .collect();
            assert_eq!(
                Leaper::WhitePawn.attacks(square),
                white,
                "white pawn {square}"
            );

            let black: Bitboard = [square + Diagonal::SouthEast, square + Diagonal::SouthWest]
                .into_iter()
                .flatten()
                .collect();
            assert_eq!(
                Leaper::BlackPawn.attacks(square),
                black,
                "black pawn {square}"
            );
        }
    }
}
