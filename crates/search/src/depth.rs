use derive_more::Display;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct Depth(u8);

impl Depth {
    pub const ZERO: Depth = Depth(0);

    #[must_use]
    pub const fn new(plies: u8) -> Depth {
        Depth(plies)
    }

    #[must_use]
    pub const fn plies(self) -> u8 {
        self.0
    }

    #[must_use]
    pub const fn incremented(self) -> Depth {
        Depth(self.0.saturating_add(1))
    }

    #[must_use]
    pub const fn decremented(self) -> Option<Depth> {
        match self.0.checked_sub(1) {
            Some(plies) => Some(Depth(plies)),
            None => None,
        }
    }
}
