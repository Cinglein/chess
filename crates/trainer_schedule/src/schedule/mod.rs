mod batch_count;
mod decay;
mod learning_rate;
mod net_id;
mod superbatch_count;
mod wdl_weight;

use batch_count::BatchCount;
use bullet_lib::trainer::schedule::TrainingSchedule;
use bullet_lib::trainer::schedule::lr::StepLR;
use bullet_lib::trainer::schedule::wdl::ConstantWDL;
use decay::Decay;
use eval::NetworkFormat;
use learning_rate::LearningRate;
pub use net_id::NetId;
use newtype::Newtype;
use serde::{Deserialize, Serialize};
use superbatch_count::SuperbatchCount;
use wdl_weight::WdlWeight;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Schedule {
    batch_size: usize,
    batches_per_superbatch: BatchCount,
    superbatches: SuperbatchCount,
    save_every: SuperbatchCount,
    learning_rate: LearningRate,
    learning_rate_decay: Decay,
    decay_every: SuperbatchCount,
    wdl: WdlWeight,
}

impl Schedule {
    #[must_use]
    pub fn training_schedule(&self, net_id: &NetId) -> TrainingSchedule<StepLR, ConstantWDL> {
        TrainingSchedule {
            net_id: net_id.clone().into_inner(),
            eval_scale: NetworkFormat::EVAL_SCALE.per_unit(),
            steps: self
                .batches_per_superbatch
                .steps(self.batch_size, self.superbatches),
            wdl_scheduler: self.wdl.constant(),
            lr_scheduler: self
                .learning_rate
                .stepped(self.learning_rate_decay, self.decay_every),
            save_rate: self.save_every.count(),
        }
    }
}
