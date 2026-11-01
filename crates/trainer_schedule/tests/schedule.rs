use trainer_schedule::{NetId, Schedule};
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
    let training = schedule.training_schedule(&NetId::new(NET.to_owned()));
    let rates = [BEFORE_DROP, AFTER_DROP].map(|superbatch| {
        (
            training.lr(FIRST_BATCH, superbatch),
            training.should_save(superbatch),
        )
    });
    assert_eq!(
        (rates, training.steps.end_superbatch),
        ([(START, false), (DROPPED, true)], LAST)
    );
}
