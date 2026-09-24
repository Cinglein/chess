use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Measurement {
    kind: &'static str,
    actual: usize,
    limit: usize,
}

impl Measurement {
    pub(super) const fn new(kind: &'static str, actual: usize, limit: usize) -> Measurement {
        Measurement {
            kind,
            actual,
            limit,
        }
    }

    pub(super) const fn exceeded(&self) -> bool {
        self.actual > self.limit
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "has {} {}, at most {} allowed",
            self.actual, self.kind, self.limit
        )
    }
}
