use std::num::NonZeroUsize;
use std::thread;

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkerCount(usize);

impl WorkerCount {
    #[must_use]
    pub fn one_per_core() -> WorkerCount {
        WorkerCount(
            thread::available_parallelism()
                .unwrap_or(NonZeroUsize::MIN)
                .get(),
        )
    }

    #[must_use]
    pub fn at_least_one(self) -> usize {
        self.0.max(NonZeroUsize::MIN.get())
    }
}

#[cfg(test)]
mod tests {
    use super::WorkerCount;

    const THREE: WorkerCount = WorkerCount(3);
    const NONE: WorkerCount = WorkerCount(0);
    const ONE: WorkerCount = WorkerCount(1);

    #[test]
    fn a_worker_count_keeps_its_value_but_never_falls_below_one() {
        assert_eq!((THREE.at_least_one(), NONE.at_least_one()), (3, 1));
        assert!(WorkerCount::one_per_core() >= ONE);
    }
}
