use board::NodeCount;
use game::{Flag, Rules, Thinking};
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

#[test]
fn rules_think_the_way_they_were_told_to() {
    let fixed = Thinking::FixedNodes(NodeCount::ONE);
    assert_eq!(Rules::DEFAULT.thinking_by(fixed).thinking(), fixed);
    assert_eq!(Rules::at_fixed_nodes(NodeCount::ONE).thinking(), fixed);
}
