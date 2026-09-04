use core::fmt;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use strum::{EnumCount, VariantArray};

use crate::diagonal::Diagonal;
use crate::direction::Direction;
use crate::file::File;
use crate::orthogonal::Orthogonal;
use crate::rank::Rank;
use crate::square::Square;
use crate::square_iter::SquareIter;
use crate::subset_iter::SubsetIter;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);

    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Bitboard(bits)
    }

    #[must_use]
    pub const fn from_square(square: Square) -> Bitboard {
        Bitboard(1 << square as u8)
    }

    #[must_use]
    pub const fn file(file: File) -> Bitboard {
        let mut squares = Bitboard::EMPTY;
        let mut index = 0;
        while index < Rank::COUNT {
            squares = squares.with(Square::new(file, Rank::VARIANTS[index]));
            index += 1;
        }
        squares
    }

    #[must_use]
    pub const fn rank(rank: Rank) -> Bitboard {
        let mut squares = Bitboard::EMPTY;
        let mut index = 0;
        while index < File::COUNT {
            squares = squares.with(Square::new(File::VARIANTS[index], rank));
            index += 1;
        }
        squares
    }

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    pub const fn contains(self, square: Square) -> bool {
        self.0 & Self::from_square(square).0 != 0
    }

    #[must_use]
    pub const fn with(self, square: Square) -> Bitboard {
        Bitboard(self.0 | Self::from_square(square).0)
    }

    #[must_use]
    pub const fn without(self, square: Square) -> Bitboard {
        Bitboard(self.0 & !Self::from_square(square).0)
    }

    #[must_use]
    pub const fn union(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }

    #[must_use]
    pub const fn intersection(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }

    #[must_use]
    pub const fn difference(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & !other.0)
    }

    #[must_use]
    pub const fn complement(self) -> Bitboard {
        Bitboard(!self.0)
    }

    #[must_use]
    pub fn least_significant_bit(self) -> Option<Square> {
        u8::try_from(self.0.trailing_zeros())
            .ok()
            .and_then(Square::from_repr)
    }

    #[must_use]
    pub const fn without_least_significant_bit(self) -> Bitboard {
        Bitboard(self.0 & self.0.wrapping_sub(1))
    }

    #[must_use]
    pub const fn subset_after(self, subset: Bitboard) -> Bitboard {
        Bitboard(subset.0.wrapping_sub(self.0) & self.0)
    }

    #[must_use]
    pub const fn subsets(self) -> SubsetIter {
        SubsetIter::new(self)
    }

    #[must_use]
    pub const fn shift(self, direction: Direction) -> Bitboard {
        let not_file_a = self.difference(Self::file(File::A)).0;
        let not_file_h = self.difference(Self::file(File::H)).0;
        Bitboard(match direction {
            Direction::Orthogonal(Orthogonal::North) => self.0 << File::COUNT,
            Direction::Orthogonal(Orthogonal::South) => self.0 >> File::COUNT,
            Direction::Orthogonal(Orthogonal::East) => not_file_h << 1,
            Direction::Orthogonal(Orthogonal::West) => not_file_a >> 1,
            Direction::Diagonal(Diagonal::NorthEast) => not_file_h << (File::COUNT + 1),
            Direction::Diagonal(Diagonal::NorthWest) => not_file_a << (File::COUNT - 1),
            Direction::Diagonal(Diagonal::SouthEast) => not_file_h >> (File::COUNT - 1),
            Direction::Diagonal(Diagonal::SouthWest) => not_file_a >> (File::COUNT + 1),
        })
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, other: Bitboard) -> Bitboard {
        self.intersection(other)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, other: Bitboard) -> Bitboard {
        self.union(other)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ other.0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        self.complement()
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Bitboard) {
        self.0 &= other.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) {
        self.0 |= other.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Bitboard) {
        self.0 ^= other.0;
    }
}

impl From<Square> for Bitboard {
    fn from(square: Square) -> Bitboard {
        Bitboard::from_square(square)
    }
}

impl FromIterator<Square> for Bitboard {
    fn from_iter<I: IntoIterator<Item = Square>>(squares: I) -> Bitboard {
        squares.into_iter().fold(Bitboard::EMPTY, Bitboard::with)
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = SquareIter;

    fn into_iter(self) -> SquareIter {
        SquareIter::new(self)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Bitboard({:#018x})", self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut squares = self.into_iter();
        if let Some(first) = squares.next() {
            write!(formatter, "{first}")?;
        }
        for square in squares {
            write!(formatter, " {square}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use proptest::prelude::*;
    use strum::IntoEnumIterator;

    use super::Bitboard;
    use crate::direction::Direction;
    use crate::rank::Rank;
    use crate::square::Square;

    #[test]
    fn every_shift_agrees_with_stepping_each_square() {
        for square in Square::iter() {
            for direction in Direction::iter() {
                let expected = (square + direction).map_or(Bitboard::EMPTY, Bitboard::from_square);
                assert_eq!(
                    Bitboard::from_square(square).shift(direction),
                    expected,
                    "{square} {direction:?}"
                );
            }
        }
    }

    #[test]
    fn set_algebra_iteration_and_display_agree() {
        proptest!(|(left: u64, right: u64)| {
            let (left, right) = (Bitboard::from_bits(left), Bitboard::from_bits(right));
            prop_assert_eq!((left | right).count() + (left & right).count(), left.count() + right.count());
            let squares: Vec<Square> = left.into_iter().collect();
            prop_assert!(squares.is_sorted() && squares.iter().copied().collect::<Bitboard>() == left);
            let listed: Vec<String> = squares.iter().map(ToString::to_string).collect();
            prop_assert_eq!(left.to_string(), listed.join(" "));
        });
    }

    #[test]
    fn subsets_of_a_mask_are_its_distinct_sub_bitboards() {
        proptest!(|(bits: u64)| {
            let mask = Bitboard::from_bits(bits) & Bitboard::rank(Rank::One);
            let subsets: Vec<Bitboard> = mask.subsets().collect();
            prop_assert_eq!(subsets.len(), 1 << mask.count());
            prop_assert_eq!(subsets.iter().collect::<HashSet<_>>().len(), subsets.len());
            prop_assert!(subsets.iter().all(|subset| subset.difference(mask).is_empty()));
        });
    }
}
