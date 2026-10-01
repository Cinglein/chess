#[derive(Clone, Copy)]
pub(crate) struct SplitMix64 {
    state: u64,
    output: u64,
}

impl SplitMix64 {
    const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
    const FIRST_MIXER: u64 = 0xBF58_476D_1CE4_E5B9;
    const SECOND_MIXER: u64 = 0x94D0_49BB_1331_11EB;
    pub(crate) const SEEDED: SplitMix64 = SplitMix64::new(0x0C4E_55B0_A4D6_4C10);

    pub(crate) const fn new(seed: u64) -> SplitMix64 {
        SplitMix64 {
            state: seed,
            output: 0,
        }
    }

    pub(crate) const fn next(self) -> SplitMix64 {
        let state = self.state.wrapping_add(Self::GOLDEN_GAMMA);
        let mixed = (state ^ (state >> 30)).wrapping_mul(Self::FIRST_MIXER);
        let mixed = (mixed ^ (mixed >> 27)).wrapping_mul(Self::SECOND_MIXER);
        SplitMix64 {
            state,
            output: mixed ^ (mixed >> 31),
        }
    }

    pub(crate) const fn output(self) -> u64 {
        self.output
    }
}

#[cfg(test)]
mod tests {
    use super::SplitMix64;

    const REFERENCE_STREAM: [&str; 3] = ["e220a8397b1dcdaf", "6e789e6aa1b965f4", "6c45d188009454f"];

    #[test]
    fn seed_zero_reproduces_the_published_splitmix64_stream() {
        let stream = REFERENCE_STREAM
            .iter()
            .scan(SplitMix64::new(0), |generator, _| {
                *generator = generator.next();
                Some(format!("{:x}", generator.output()))
            });
        assert!(stream.eq(REFERENCE_STREAM.iter().map(ToString::to_string)));
    }
}
