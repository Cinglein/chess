use std::num::NonZeroUsize;
use std::thread;

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ThreadCount(usize);

impl ThreadCount {
    #[must_use]
    pub const fn count(self) -> usize {
        self.0
    }
}

impl Default for ThreadCount {
    fn default() -> ThreadCount {
        ThreadCount(
            thread::available_parallelism()
                .unwrap_or(NonZeroUsize::MIN)
                .get(),
        )
    }
}
