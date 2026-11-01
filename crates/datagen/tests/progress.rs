use datagen::{PositionCount, Progress};
use std::time::{Duration, Instant};

const TARGET: PositionCount = PositionCount::new(4);
const TWO: PositionCount = PositionCount::new(2);
const INTERVAL: Duration = Duration::from_secs(5);

#[test]
fn a_report_is_due_once_the_interval_has_passed_since_the_last_acknowledgement() {
    let before = Instant::now();
    let progress = Progress::towards(TARGET);
    let later = Instant::now() + INTERVAL;
    let due = [before, later, later + INTERVAL].map(|at| progress.is_due(at));
    assert_eq!(due, [false, true, true]);
    assert!(!progress.acknowledged(later).is_due(later));
}

#[test]
fn progress_prints_the_positions_written_against_the_target() {
    let text = Progress::towards(TARGET).advanced(TWO).to_string();
    assert!(text.starts_with("2/4 positions"), "{text}");
}
