use board::NodeCount;
use core::time::Duration;
const TWO: NodeCount = NodeCount::new(2);
const FORTY: NodeCount = NodeCount::new(40);
const FIFTY_MILLIS: Duration = Duration::from_millis(50);

#[test]
fn nodes_per_second_scales_a_count_by_the_elapsed_milliseconds() {
    assert_eq!(TWO.per_second(FIFTY_MILLIS), FORTY);
}
