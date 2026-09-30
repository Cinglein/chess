use std::fmt;

use sysinfo::System;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AvailableGib(u64);

impl AvailableGib {
    pub const MINIMUM: AvailableGib = AvailableGib(16);

    pub fn measured() -> AvailableGib {
        let mut system = System::new();
        system.refresh_memory();
        AvailableGib(system.available_memory() >> 30)
    }
}

impl fmt::Display for AvailableGib {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} GiB", self.0)
    }
}
