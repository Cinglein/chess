use std::fmt;

use super::elo_delta::EloDelta;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EloEstimate {
    low: EloDelta,
    point: EloDelta,
    high: EloDelta,
}

impl EloEstimate {
    pub(super) const fn new(low: EloDelta, point: EloDelta, high: EloDelta) -> EloEstimate {
        EloEstimate { low, point, high }
    }

    #[must_use]
    pub fn excludes_even(&self) -> bool {
        self.low > EloDelta::EVEN || self.high < EloDelta::EVEN
    }
}

impl fmt::Display for EloEstimate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Elo {} ({} to {})",
            self.point, self.low, self.high
        )?;
        formatter.write_str(if self.excludes_even() {
            " decisive"
        } else {
            " inconclusive"
        })
    }
}
