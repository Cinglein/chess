use std::num::NonZeroUsize;
use std::thread;

use derive_more::{Display, Into};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Into, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ThreadCount(usize);

impl Default for ThreadCount {
    fn default() -> ThreadCount {
        ThreadCount(
            thread::available_parallelism()
                .unwrap_or(NonZeroUsize::MIN)
                .get(),
        )
    }
}
