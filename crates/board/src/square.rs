use core::fmt;
use core::str::FromStr;

use strum::{EnumCount, EnumIter, FromRepr, ParseError, VariantArray};

use crate::direction::Direction;
use crate::file::File;
use crate::rank::Rank;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumCount,
    EnumIter,
    FromRepr,
    VariantArray,
)]
#[repr(u8)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    #[must_use]
    pub const fn new(file: File, rank: Rank) -> Square {
        Self::VARIANTS[rank.index() * File::COUNT + file.index()]
    }

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn file(self) -> File {
        File::VARIANTS[self.index() % File::COUNT]
    }

    #[must_use]
    pub const fn rank(self) -> Rank {
        Rank::VARIANTS[self.index() / File::COUNT]
    }

    #[must_use]
    pub const fn translate(self, file_delta: i8, rank_delta: i8) -> Option<Square> {
        let Some(file) = (self.file() as u8).checked_add_signed(file_delta) else {
            return None;
        };
        let Some(rank) = (self.rank() as u8).checked_add_signed(rank_delta) else {
            return None;
        };
        match (File::from_repr(file), Rank::from_repr(rank)) {
            (Some(file), Some(rank)) => Some(Self::new(file, rank)),
            _ => None,
        }
    }

    #[must_use]
    pub const fn offset(self, direction: Direction) -> Option<Square> {
        self.translate(direction.file_delta(), direction.rank_delta())
    }
}

impl fmt::Display for Square {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.file(), self.rank())
    }
}

impl FromStr for Square {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Square, ParseError> {
        let (file, rank) = text
            .split_at_checked(1)
            .ok_or(ParseError::VariantNotFound)?;
        Ok(Square::new(file.parse()?, rank.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use strum::{EnumCount, IntoEnumIterator};

    use super::Square;
    use crate::direction::Direction;
    use crate::file::File;
    use crate::rank::Rank;

    #[test]
    fn squares_are_numbered_rank_by_rank_from_a1() {
        assert_eq!(Square::A1.index(), 0);
        assert_eq!(Square::H1.index(), 7);
        assert_eq!(Square::A2.index(), 8);
        assert_eq!(Square::H8.index(), 63);
        assert_eq!(Square::COUNT, 64);
        assert_eq!(Square::new(File::E, Rank::Four), Square::E4);
        assert_eq!(Square::E4.file(), File::E);
        assert_eq!(Square::E4.rank(), Rank::Four);
    }

    #[test]
    fn every_square_roundtrips_through_its_index_and_coordinates() {
        for (index, square) in (0u8..).zip(Square::iter()) {
            assert_eq!(square.index(), usize::from(index));
            assert_eq!(Square::from_repr(index), Some(square));
            assert_eq!(Square::new(square.file(), square.rank()), square);
        }
        assert_eq!(Square::from_repr(64), None);
    }

    #[test]
    fn squares_display_and_parse_in_algebraic_notation() {
        assert_eq!(Square::E4.to_string(), "e4");
        assert_eq!("e4".parse(), Ok(Square::E4));
        assert_eq!("h8".parse(), Ok(Square::H8));
        assert!("e9".parse::<Square>().is_err());
        assert!("i1".parse::<Square>().is_err());
        assert!("e".parse::<Square>().is_err());
        assert!("".parse::<Square>().is_err());
        assert!("e44".parse::<Square>().is_err());
    }

    #[test]
    fn stepping_off_the_board_yields_none() {
        assert_eq!(Square::E4.offset(Direction::North), Some(Square::E5));
        assert_eq!(Square::E4.offset(Direction::SouthWest), Some(Square::D3));
        assert_eq!(Square::A1.offset(Direction::West), None);
        assert_eq!(Square::A1.offset(Direction::South), None);
        assert_eq!(Square::H8.offset(Direction::NorthEast), None);
        assert_eq!(Square::H4.translate(1, 2), None);
        assert_eq!(Square::B1.translate(1, 2), Some(Square::C3));
    }
}
