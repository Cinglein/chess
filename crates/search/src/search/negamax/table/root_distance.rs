#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RootDistance(u8);

impl RootDistance {
    pub(crate) const ROOT: RootDistance = RootDistance(0);

    pub(crate) const fn plies(self) -> u8 {
        self.0
    }

    pub(crate) const fn deeper(self) -> RootDistance {
        RootDistance(self.0.saturating_add(1))
    }
}
