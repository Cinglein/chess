use derive_more::Display;
use newtype::Newtype;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Newtype)]
pub struct Quantisation(i16);

impl Quantisation {
    #[must_use]
    pub const fn factor(self) -> i16 {
        self.0
    }

    #[must_use]
    pub const fn times(self, other: Quantisation) -> Quantisation {
        Quantisation(self.0 * other.0)
    }
}
