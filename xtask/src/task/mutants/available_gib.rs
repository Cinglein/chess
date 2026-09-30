use std::fmt;

use sysinfo::System;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AvailableGib(u64);

impl AvailableGib {
    const PER_COMPILER_TASK: u64 = 2;
    const FOR_THE_TESTS: u64 = 2;

    pub fn measured() -> AvailableGib {
        let mut system = System::new();
        system.refresh_memory();
        AvailableGib(system.available_memory() >> 30)
    }

    pub fn needed_for(compiler_tasks: usize) -> AvailableGib {
        let tasks = u64::try_from(compiler_tasks).unwrap_or(u64::MAX);
        AvailableGib(tasks.saturating_mul(Self::PER_COMPILER_TASK) + Self::FOR_THE_TESTS)
    }
}

impl fmt::Display for AvailableGib {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} GiB", self.0)
    }
}
