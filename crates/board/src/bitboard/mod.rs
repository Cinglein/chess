mod square_iter;
mod subset_iter;

pub use square_iter::SquareIter;
pub use subset_iter::SubsetIter;

use core::fmt;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use itertools::Itertools;
use strum::{EnumCount, VariantArray};

use crate::direction::Direction;
use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);
    pub const LIGHT_SQUARES: Bitboard = Bitboard(0x55AA_55AA_55AA_55AA);

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
        let mut ranks = Rank::VARIANTS;
        while let [rank, rest @ ..] = ranks {
            squares = squares.including(Square::new(file, *rank));
            ranks = rest;
        }
        squares
    }

    #[must_use]
    pub const fn rank(rank: Rank) -> Bitboard {
        let mut squares = Bitboard::EMPTY;
        let mut files = File::VARIANTS;
        while let [file, rest @ ..] = files {
            squares = squares.including(Square::new(*file, rank));
            files = rest;
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
    pub const fn including(self, square: Square) -> Bitboard {
        Bitboard(self.0 | Self::from_square(square).0)
    }

    #[must_use]
    pub const fn excluding(self, square: Square) -> Bitboard {
        Bitboard(self.0 & !Self::from_square(square).0)
    }

    #[must_use]
    pub const fn union(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }

    #[must_use]
    pub const fn disjoint_union(self, other: Bitboard) -> Bitboard {
        debug_assert!(self.intersection(other).is_empty());
        self.union(other)
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
    pub const fn shift(self, direction: Direction) -> Bitboard {
        let vertical = match direction.rank_step() {
            1 => self.0 << File::COUNT,
            -1 => self.0 >> File::COUNT,
            _ => self.0,
        };
        let horizontal = match direction.file_step() {
            1 => (vertical & !Self::file(File::H).0) << 1,
            -1 => (vertical & !Self::file(File::A).0) >> 1,
            _ => vertical,
        };
        Bitboard(horizontal)
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
        squares
            .into_iter()
            .fold(Bitboard::EMPTY, Bitboard::including)
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
        write!(formatter, "{}", self.into_iter().format(" "))
    }
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for Bitboard {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Bitboard>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        proptest::arbitrary::any::<u64>()
            .prop_map(Bitboard::from_bits)
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use proptest::prelude::*;
    use strum::{EnumCount, IntoEnumIterator};

    use super::{Bitboard, SubsetIter};
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
        proptest!(|(left: Bitboard, right: Bitboard)| {
            let mut toggled = left;
            toggled ^= right;
            prop_assert_eq!(
                ((left | right).count() + (left & right).count(), toggled, Bitboard::from_bits(left.bits())),
                (left.count() + right.count(), left ^ right, left)
            );
            let squares: Vec<Square> = left.into_iter().take(Square::COUNT + 1).collect();
            prop_assert!(squares.is_sorted() && squares.iter().copied().collect::<Bitboard>() == left && squares.len() == left.into_iter().len());
            let listed: Vec<String> = squares.iter().map(ToString::to_string).collect();
            let (debugged, hex) = (format!("{left:?}"), format!("{:x}", left.bits()));
            prop_assert!(left.to_string() == listed.join(" ") && debugged.contains(&hex));
        });
    }

    #[test]
    fn a_square_joins_and_leaves_a_set_and_subsets_of_a_mask_are_its_distinct_sub_bitboards() {
        proptest!(|(set: Bitboard, square: Square)| {
            prop_assert!(set.including(square).contains(square) && set.excluding(square) == set & !Bitboard::from(square));
            let mask = set & Bitboard::rank(Rank::One);
            let subsets: Vec<Bitboard> = SubsetIter::new(mask).take((1 << mask.count()) + 1).collect();
            prop_assert_eq!(subsets.len(), 1 << mask.count());
            prop_assert!(subsets.iter().collect::<HashSet<_>>().len() == subsets.len() && subsets.iter().all(|subset| subset.difference(mask).is_empty()));
        });
    }
}
