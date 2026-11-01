use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct EloDelta(f64);

impl EloDelta {
    pub const EVEN: EloDelta = EloDelta(0.0);

    pub(super) const fn new(elo: f64) -> EloDelta {
        EloDelta(elo)
    }
}

impl fmt::Display for EloDelta {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:+.0}", self.0)
    }
}
