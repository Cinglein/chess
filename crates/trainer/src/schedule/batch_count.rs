use bullet_lib::trainer::schedule::TrainingSteps;
use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::superbatch_count::SuperbatchCount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BatchCount(usize);

impl BatchCount {
    #[must_use]
    pub const fn steps(self, batch_size: usize, superbatches: SuperbatchCount) -> TrainingSteps {
        TrainingSteps {
            batch_size,
            batches_per_superbatch: self.0,
            start_superbatch: SuperbatchCount::FIRST.count(),
            end_superbatch: superbatches.count(),
        }
    }
}
