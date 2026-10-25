use std::fmt;

use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AvailableGib(u64);

impl AvailableGib {
    const GIB_SHIFT: u32 = 30;

    #[must_use]
    pub const fn from_bytes(bytes: u64) -> AvailableGib {
        AvailableGib(bytes >> Self::GIB_SHIFT)
    }

    #[must_use]
    pub fn measured() -> AvailableGib {
        let mut system = System::new();
        system.refresh_memory();
        Self::from_bytes(system.available_memory())
    }

    #[must_use]
    pub const fn bytes(self) -> u64 {
        self.0 << Self::GIB_SHIFT
    }

    #[must_use]
    pub const fn minus(self, other: AvailableGib) -> AvailableGib {
        AvailableGib(self.0.saturating_sub(other.0))
    }

    #[must_use]
    pub fn divided_by(self, other: AvailableGib) -> usize {
        usize::try_from(self.0 / other.0).unwrap_or(usize::MAX)
    }
}

impl fmt::Display for AvailableGib {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} GiB", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::AvailableGib;

    const SIX: AvailableGib = AvailableGib(6);
    const TWO: AvailableGib = AvailableGib(2);
    const THIRDS: usize = 3;

    #[test]
    fn gibibytes_round_down_from_bytes_divide_whole_and_print_their_unit() {
        assert_eq!(AvailableGib::from_bytes(SIX.bytes() + 1), SIX);
        assert_eq!(SIX.divided_by(TWO), THIRDS);
        assert_eq!(SIX.to_string(), "6 GiB");
    }
}
