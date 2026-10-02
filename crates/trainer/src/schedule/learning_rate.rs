use bullet_lib::trainer::schedule::lr::StepLR;
use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::decay::Decay;
use super::superbatch_count::SuperbatchCount;

#[derive(Clone, Copy, Debug, PartialEq, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LearningRate(f32);

impl LearningRate {
    #[must_use]
    pub const fn stepped(self, decay: Decay, every: SuperbatchCount) -> StepLR {
        StepLR {
            start: self.0,
            gamma: decay.factor(),
            step: every.count(),
        }
    }
}
