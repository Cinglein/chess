use core::fmt;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::square::{Direction, File, Rank, Square};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);

    #[must_use]
    pub const fn from_bits(bits: u64) -> Bitboard {
        Bitboard(bits)
    }

    #[must_use]
    pub const fn from_square(square: Square) -> Bitboard {
        Bitboard(1 << square.index())
    }

    #[must_use]
    pub const fn file(file: File) -> Bitboard {
        Bitboard(0x0101_0101_0101_0101 << file.index())
    }

    #[must_use]
    pub const fn rank(rank: Rank) -> Bitboard {
        Bitboard(0xFF << (rank.index() * 8))
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
    pub const fn first(self) -> Option<Square> {
        Square::from_index(self.0.trailing_zeros() as usize)
    }

    #[must_use]
    pub const fn without_first(self) -> Bitboard {
        Bitboard(self.0 & self.0.wrapping_sub(1))
    }

    #[must_use]
    pub const fn shift(self, direction: Direction) -> Bitboard {
        let not_file_a = self.difference(Self::file(File::A)).0;
        let not_file_h = self.difference(Self::file(File::H)).0;
        Bitboard(match direction {
            Direction::North => self.0 << 8,
            Direction::South => self.0 >> 8,
            Direction::East => not_file_h << 1,
            Direction::West => not_file_a >> 1,
            Direction::NorthEast => not_file_h << 9,
            Direction::NorthWest => not_file_a << 7,
            Direction::SouthEast => not_file_h >> 7,
            Direction::SouthWest => not_file_a >> 9,
        })
    }

    #[must_use]
    pub const fn squares(self) -> Squares {
        Squares(self)
    }

    #[must_use]
    pub const fn subset_after(self, subset: Bitboard) -> Bitboard {
        Bitboard(subset.0.wrapping_sub(self.0) & self.0)
    }

    #[must_use]
    pub const fn subsets(self) -> Subsets {
        Subsets {
            mask: self,
            next: Some(Bitboard::EMPTY),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Subsets {
    mask: Bitboard,
    next: Option<Bitboard>,
}

impl Iterator for Subsets {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Bitboard> {
        let subset = self.next?;
        let following = self.mask.subset_after(subset);
        self.next = if following.is_empty() {
            None
        } else {
            Some(following)
        };
        Some(subset)
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
    type IntoIter = Squares;

    fn into_iter(self) -> Squares {
        self.squares()
    }
}

#[derive(Clone, Debug)]
pub struct Squares(Bitboard);

impl Iterator for Squares {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        let square = self.0.first()?;
        self.0 = self.0.without_first();
        Some(square)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.0.count() as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Squares {}

impl fmt::Debug for Bitboard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Bitboard({:#018x})", self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in Rank::ALL.iter().rev() {
            for file in File::ALL {
                let marker = if self.contains(Square::new(file, *rank)) {
                    'X'
                } else {
                    '.'
                };
                formatter.write_str(if file == File::A { "" } else { " " })?;
                write!(formatter, "{marker}")?;
            }
            writeln!(formatter)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Bitboard;
    use crate::square::{Direction, File, Rank, Square};

    #[test]
    fn a_bitboard_is_a_set_of_squares() {
        let set = Bitboard::EMPTY.with(Square::E4).with(Square::A1);
        assert!(set.contains(Square::E4));
        assert!(set.contains(Square::A1));
        assert!(!set.contains(Square::E5));
        assert_eq!(set.count(), 2);
        assert_eq!(set.without(Square::E4), Bitboard::from_square(Square::A1));
    }

    #[test]
    fn set_operators_follow_set_semantics() {
        let left = Bitboard::rank(Rank::One);
        let right = Bitboard::file(File::A);
        assert_eq!((left | right).count(), 15);
        assert_eq!(left & right, Bitboard::from_square(Square::A1));
        assert_eq!((left ^ right).count(), 14);
        assert_eq!(left.difference(right).count(), 7);
        assert_eq!(!Bitboard::EMPTY, Bitboard::FULL);
    }

    #[test]
    fn iteration_yields_squares_from_a1_towards_h8() {
        let set: Bitboard = [Square::H8, Square::A1, Square::E4].into_iter().collect();
        let squares: Vec<Square> = set.into_iter().collect();
        assert_eq!(squares, vec![Square::A1, Square::E4, Square::H8]);
        assert_eq!(set.squares().len(), 3);
        assert_eq!(Bitboard::EMPTY.first(), None);
    }

    #[test]
    fn shifting_east_or_west_never_wraps_around_the_board() {
        let h_file = Bitboard::file(File::H);
        assert_eq!(h_file.shift(Direction::East), Bitboard::EMPTY);
        assert_eq!(h_file.shift(Direction::West), Bitboard::file(File::G));
        let a_file = Bitboard::file(File::A);
        assert_eq!(a_file.shift(Direction::West), Bitboard::EMPTY);
        assert_eq!(a_file.shift(Direction::NorthWest), Bitboard::EMPTY);
        assert_eq!(a_file.shift(Direction::SouthWest), Bitboard::EMPTY);
    }

    #[test]
    fn shifting_north_off_the_eighth_rank_drops_squares() {
        let eighth = Bitboard::rank(Rank::Eight);
        assert_eq!(eighth.shift(Direction::North), Bitboard::EMPTY);
        assert_eq!(
            Bitboard::from_square(Square::E4).shift(Direction::NorthEast),
            Bitboard::from_square(Square::F5)
        );
    }

    #[test]
    fn every_shift_agrees_with_stepping_each_square() {
        for square in Square::ALL {
            for direction in Direction::ALL {
                let expected = square
                    .offset(direction)
                    .map_or(Bitboard::EMPTY, Bitboard::from_square);
                assert_eq!(
                    Bitboard::from_square(square).shift(direction),
                    expected,
                    "{square} {direction:?}"
                );
            }
        }
    }

    #[test]
    fn subsets_enumerate_every_combination_of_a_mask_once() {
        let mask = Bitboard::from_square(Square::A1)
            .with(Square::D4)
            .with(Square::H8);
        let subsets: Vec<Bitboard> = mask.subsets().collect();
        assert_eq!(subsets.len(), 8);
        assert_eq!(subsets[0], Bitboard::EMPTY);
        assert_eq!(subsets[7], mask);
        for (index, subset) in subsets.iter().enumerate() {
            assert_eq!(subset.difference(mask), Bitboard::EMPTY);
            assert!(!subsets[..index].contains(subset));
        }
        assert_eq!(Bitboard::EMPTY.subsets().count(), 1);
    }

    #[test]
    fn display_prints_the_eighth_rank_first() {
        let set = Bitboard::from_square(Square::A8).with(Square::H1);
        let text = set.to_string();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 8);
        assert_eq!(lines[0], "X . . . . . . .");
        assert_eq!(lines[7], ". . . . . . . X");
    }
}
