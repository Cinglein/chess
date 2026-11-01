use board::NodeCount;
use game::{Flag, Thinking};
use std::time::Duration;

const REMAINING: Duration = Duration::from_millis(5);
const OVERRUN: Duration = REMAINING.saturating_add(Duration::from_nanos(1));
const CASES: [(Thinking, Duration, Flag); 3] = [
    (Thinking::Timed, REMAINING, Flag::Standing),
    (Thinking::Timed, OVERRUN, Flag::Fallen),
    (
        Thinking::FixedNodes(NodeCount::ONE),
        OVERRUN,
        Flag::Standing,
    ),
];

#[test]
fn a_timed_mover_forfeits_only_by_overrunning_the_clock_and_a_node_limited_mover_never_does() {
    for (thinking, spent, flag) in CASES {
        assert_eq!(thinking.flag(spent, REMAINING), flag);
    }
}
