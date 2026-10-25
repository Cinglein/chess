use derive_more::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub struct Quantisation(i16);

impl Quantisation {
    #[must_use]
    pub const fn new(factor: i16) -> Quantisation {
        Quantisation(factor)
    }

    #[must_use]
    pub const fn factor(self) -> i16 {
        self.0
    }

    #[must_use]
    pub const fn times(self, other: Quantisation) -> Quantisation {
        Quantisation(self.0 * other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Quantisation;

    const THREE: Quantisation = Quantisation::new(3);
    const FOUR: Quantisation = Quantisation::new(4);

    #[test]
    fn quantisations_multiply_when_two_quantised_layers_meet() {
        assert_eq!(THREE.times(FOUR).factor(), 12);
    }
}
