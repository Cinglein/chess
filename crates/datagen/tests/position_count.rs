use datagen::PositionCount;
use std::time::Duration;
const TEN: PositionCount = PositionCount::new(10);
const FIVE: PositionCount = PositionCount::new(5);
const TWO_SECONDS: Duration = Duration::from_secs(2);

#[test]
fn a_rate_divides_by_whole_seconds_and_a_run_under_a_second_counts_as_one() {
    assert_eq!(
        (TEN.per_second(TWO_SECONDS), TEN.per_second(Duration::ZERO)),
        (FIVE, TEN)
    );
}
