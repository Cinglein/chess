use std::fmt;

use sysinfo::System;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AvailableGib(u64);

impl AvailableGib {
    pub const GRANTED: AvailableGib = AvailableGib(8);
    pub const RESERVE: AvailableGib = AvailableGib(1);
    const PER_COMPILER_TASK: u64 = 2;
    const FOR_THE_TESTS: u64 = 2;
    const GIB_SHIFT: u32 = 30;

    pub const fn from_bytes(bytes: u64) -> AvailableGib {
        AvailableGib(bytes >> Self::GIB_SHIFT)
    }

    pub fn measured() -> AvailableGib {
        let mut system = System::new();
        system.refresh_memory();
        Self::from_bytes(system.available_memory())
    }

    pub const fn bytes(self) -> u64 {
        self.0 << Self::GIB_SHIFT
    }

    pub const fn minus_reserve(self) -> AvailableGib {
        AvailableGib(self.0.saturating_sub(Self::RESERVE.0))
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
