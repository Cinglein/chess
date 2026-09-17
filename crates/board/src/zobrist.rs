use core::ops::{BitXor, BitXorAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Zobrist(u64);

impl Zobrist {
    pub const EMPTY: Zobrist = Zobrist(0);

    #[must_use]
    pub const fn from_bits(bits: u64) -> Zobrist {
        Zobrist(bits)
    }

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn xor(self, other: Zobrist) -> Zobrist {
        Zobrist(self.0 ^ other.0)
    }
}

impl BitXor for Zobrist {
    type Output = Zobrist;

    fn bitxor(self, other: Zobrist) -> Zobrist {
        self.xor(other)
    }
}

impl BitXorAssign for Zobrist {
    fn bitxor_assign(&mut self, other: Zobrist) {
        self.0 ^= other.0;
    }
}
