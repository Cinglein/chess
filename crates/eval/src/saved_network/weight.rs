#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Weight(i16);

impl Weight {
    pub(super) const fn new(units: i16) -> Weight {
        Weight(units)
    }

    pub(super) const fn units(self) -> i16 {
        self.0
    }
}
