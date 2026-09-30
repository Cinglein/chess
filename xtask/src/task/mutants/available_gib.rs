use std::fmt;

use sysinfo::System;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AvailableGib(u64);

impl AvailableGib {
    pub const GRANTED: AvailableGib = AvailableGib(8);
    const PER_COMPILER_TASK: u64 = 2;
    const FOR_THE_TESTS: u64 = 2;

    pub fn measured() -> AvailableGib {
        let mut system = System::new();
        system.refresh_memory();
        AvailableGib(system.available_memory() >> 30)
    }

    pub const fn compiler_tasks(self) -> usize {
        (self.0.saturating_sub(Self::FOR_THE_TESTS) / Self::PER_COMPILER_TASK) as usize
    }
}

impl fmt::Display for AvailableGib {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} GiB", self.0)
    }
}
