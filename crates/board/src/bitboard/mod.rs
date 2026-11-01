mod bits;
mod set_operation;
mod square_iter;
mod subset_iter;

pub use set_operation::SetOperation;
pub use square_iter::SquareIter;
pub use subset_iter::SubsetIter;

use core::fmt;
use core::ops::{BitAnd, BitOr, BitXor, BitXorAssign, Not};

use itertools::Itertools;
use strum::VariantArray;

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
            squares = squares.combined(
                Self::from_square(Square::new(file, *rank)),
                SetOperation::Union,
            );
            ranks = rest;
        }
        squares
    }

    #[must_use]
    pub const fn rank(rank: Rank) -> Bitboard {
        let mut squares = Bitboard::EMPTY;
        let mut files = File::VARIANTS;
        while let [file, rest @ ..] = files {
            squares = squares.combined(
                Self::from_square(Square::new(*file, rank)),
                SetOperation::Union,
            );
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
    pub const fn combined(self, other: Bitboard, operation: SetOperation) -> Bitboard {
        match operation {
            SetOperation::Union => Bitboard(self.0 | other.0),
            SetOperation::DisjointUnion => {
                debug_assert!(self.0 & other.0 == 0);
                self.combined(other, SetOperation::Union)
            }
            SetOperation::Intersection => Bitboard(self.0 & other.0),
            SetOperation::Difference => Bitboard(self.0 & !other.0),
        }
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
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
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
        Bitboard(!self.0)
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
        squares.into_iter().fold(Bitboard::EMPTY, |set, square| {
            set.combined(Bitboard::from_square(square), SetOperation::Union)
        })
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

#[cfg(feature = "proptest")]
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
