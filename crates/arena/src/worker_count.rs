use std::num::NonZeroUsize;
use std::thread;

use derive_more::{Display, From};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Display, From, Deserialize)]
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
