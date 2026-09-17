pub(crate) struct SplitMix64(u64);

impl SplitMix64 {
    const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
    const FIRST_MIXER: u64 = 0xBF58_476D_1CE4_E5B9;
    const SECOND_MIXER: u64 = 0x94D0_49BB_1331_11EB;

    pub(crate) const fn new(seed: u64) -> SplitMix64 {
        SplitMix64(seed)
    }

    pub(crate) const fn next(self) -> (SplitMix64, u64) {
        let state = self.0.wrapping_add(Self::GOLDEN_GAMMA);
        let mixed = (state ^ (state >> 30)).wrapping_mul(Self::FIRST_MIXER);
        let mixed = (mixed ^ (mixed >> 27)).wrapping_mul(Self::SECOND_MIXER);
        (SplitMix64(state), mixed ^ (mixed >> 31))
    }
}
