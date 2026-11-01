pub struct XorShift(u64);

impl XorShift {
    pub fn sparse(&mut self) -> u64 {
        self.take(3).fold(u64::MAX, |bits, word| bits & word)
    }
}

impl From<u64> for XorShift {
    fn from(seed: u64) -> XorShift {
        XorShift(seed)
    }
}

impl Iterator for XorShift {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        Some(self.0.wrapping_mul(0x2545_F491_4F6C_DD1D))
    }
}
