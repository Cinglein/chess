mod batch_count;
mod decay;
mod learning_rate;
mod superbatch_count;
mod wdl_weight;

use batch_count::BatchCount;
use bullet_lib::trainer::schedule::TrainingSchedule;
use bullet_lib::trainer::schedule::lr::StepLR;
use bullet_lib::trainer::schedule::wdl::ConstantWDL;
use decay::Decay;
use eval::NetworkFormat;
use learning_rate::LearningRate;
use serde::{Deserialize, Serialize};
use superbatch_count::SuperbatchCount;
use wdl_weight::WdlWeight;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub fn training_schedule(&self, net_id: &str) -> TrainingSchedule<StepLR, ConstantWDL> {
        TrainingSchedule {
            net_id: net_id.to_owned(),
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

#[cfg(test)]
mod tests {
    use super::Schedule;

    const WRITTEN: &str = "
        batch_size = 8
        batches_per_superbatch = 4
        superbatches = 6
        save_every = 3
        learning_rate = 0.5
        learning_rate_decay = 0.1
        decay_every = 2
        wdl = 0.75
    ";
    const NET: &str = "net";
    const FIRST_BATCH: usize = 0;
    const BEFORE_DROP: usize = 2;
    const AFTER_DROP: usize = 3;
    const START: f32 = 0.5;
    const DROPPED: f32 = 0.05;
    const LAST: usize = 6;

    #[test]
    fn the_schedule_drops_the_learning_rate_on_its_step_and_saves_on_its_rate() {
        let schedule: Schedule = toml::from_str(WRITTEN).unwrap();
        let training = schedule.training_schedule(NET);
        assert_eq!(
            (
                training.lr(FIRST_BATCH, BEFORE_DROP),
                training.lr(FIRST_BATCH, AFTER_DROP),
                training.steps.end_superbatch
            ),
            (START, DROPPED, LAST)
        );
        assert_eq!(
            (
                training.should_save(BEFORE_DROP),
                training.should_save(AFTER_DROP)
            ),
            (false, true)
        );
    }
}
