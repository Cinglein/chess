use std::time::Duration;

use board::NodeCount;
use uci::{Clock, GoLimits};

use crate::flag::Flag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thinking {
    Timed,
    FixedNodes(NodeCount),
}

impl Thinking {
    #[must_use]
    pub const fn limits(&self, clock: Clock) -> GoLimits {
        match self {
            Thinking::Timed => GoLimits::Clock(clock),
            Thinking::FixedNodes(nodes) => GoLimits::Nodes(*nodes),
        }
    }

    #[must_use]
    pub fn flag(&self, spent: Duration, remaining: Duration) -> Flag {
        match self {
            Thinking::Timed if spent > remaining => Flag::Fallen,
            Thinking::Timed | Thinking::FixedNodes(_) => Flag::Standing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Thinking;
    use crate::flag::Flag;
    use board::NodeCount;
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
}
