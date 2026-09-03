use core::fmt;
use core::ops::Add;
use core::str::FromStr;

use enum_map::Enum;
use strum::{EnumCount, EnumIter, FromRepr, ParseError, VariantArray};

use crate::diagonal::Diagonal;
use crate::direction::Direction;
use crate::file::File;
use crate::orthogonal::Orthogonal;
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
    Enum,
    EnumCount,
    EnumIter,
    FromRepr,
    VariantArray,
)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub(crate) const fn from_bits(bits: u8) -> Self {
        Self::VARIANTS[bits as usize]
    }

    pub(crate) const fn into_bits(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn new(file: File, rank: Rank) -> Square {
        Self::VARIANTS[rank as usize * File::COUNT + file as usize]
    }

    #[must_use]
    pub const fn file(self) -> File {
        File::VARIANTS[self as usize % File::COUNT]
    }

    #[must_use]
    pub const fn rank(self) -> Rank {
        Rank::VARIANTS[self as usize / File::COUNT]
    }
}

impl<D: Into<Direction>> Add<D> for Square {
    type Output = Option<Square>;

    fn add(self, direction: D) -> Option<Square> {
        let (file_step, rank_step): (i8, i8) = match direction.into() {
            Direction::Orthogonal(Orthogonal::North) => (0, 1),
            Direction::Orthogonal(Orthogonal::East) => (1, 0),
            Direction::Orthogonal(Orthogonal::South) => (0, -1),
            Direction::Orthogonal(Orthogonal::West) => (-1, 0),
            Direction::Diagonal(Diagonal::NorthEast) => (1, 1),
            Direction::Diagonal(Diagonal::SouthEast) => (1, -1),
            Direction::Diagonal(Diagonal::SouthWest) => (-1, -1),
            Direction::Diagonal(Diagonal::NorthWest) => (-1, 1),
        };
        let file = File::from_repr((self.file() as u8).checked_add_signed(file_step)?)?;
        let rank = Rank::from_repr((self.rank() as u8).checked_add_signed(rank_step)?)?;
        Some(Square::new(file, rank))
    }
}

impl Add<Direction> for Option<Square> {
    type Output = Option<Square>;

    fn add(self, direction: Direction) -> Option<Square> {
        self.and_then(|square| square + direction)
    }
}

impl Add<Orthogonal> for Option<Square> {
    type Output = Option<Square>;

    fn add(self, direction: Orthogonal) -> Option<Square> {
        self.and_then(|square| square + direction)
    }
}

impl Add<Diagonal> for Option<Square> {
    type Output = Option<Square>;

    fn add(self, direction: Diagonal) -> Option<Square> {
        self.and_then(|square| square + direction)
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
    use enum_map::Enum;
    use strum::{EnumCount, IntoEnumIterator};

    use super::Square;
    use crate::diagonal::Diagonal;
    use crate::file::File;
    use crate::orthogonal::Orthogonal;
    use crate::rank::Rank;

    #[test]
    fn squares_are_numbered_rank_by_rank_from_a1() {
        assert_eq!(Square::A1.into_usize(), 0);
        assert_eq!(Square::H1.into_usize(), 7);
        assert_eq!(Square::A2.into_usize(), 8);
        assert_eq!(Square::H8.into_usize(), 63);
        assert_eq!(Square::COUNT, 64);
        assert_eq!(Square::new(File::E, Rank::Four), Square::E4);
        assert_eq!(Square::E4.file(), File::E);
        assert_eq!(Square::E4.rank(), Rank::Four);
    }

    #[test]
    fn every_square_roundtrips_through_its_index_and_coordinates() {
        for (index, square) in (0u8..).zip(Square::iter()) {
            assert_eq!(square.into_usize(), usize::from(index));
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
        assert_eq!(Square::E4 + Orthogonal::North, Some(Square::E5));
        assert_eq!(Square::E4 + Diagonal::SouthWest, Some(Square::D3));
        assert_eq!(Square::A1 + Orthogonal::West, None);
        assert_eq!(Square::A1 + Orthogonal::South, None);
        assert_eq!(Square::H8 + Diagonal::NorthEast, None);
    }

    #[test]
    fn steps_chain_through_option_so_a_knight_jump_is_two_additions() {
        assert_eq!(
            Square::B1 + Orthogonal::North + Diagonal::NorthEast,
            Some(Square::C3)
        );
        assert_eq!(Square::H4 + Orthogonal::North + Diagonal::NorthEast, None);
        assert_eq!(Square::A1 + Orthogonal::West + Orthogonal::North, None);
    }
}
