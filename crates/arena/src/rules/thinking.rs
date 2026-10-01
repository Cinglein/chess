use std::time::Duration;

use board::NodeCount;
use uci::{Clock, GoLimits};

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
    pub fn forfeits(&self, spent: Duration, remaining: Duration) -> bool {
        match self {
            Thinking::Timed => spent > remaining,
            Thinking::FixedNodes(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use board::NodeCount;

    use super::Thinking;

    const REMAINING: Duration = Duration::from_millis(5);
    const OVERRUN: Duration = Duration::from_nanos(1);

    #[test]
    fn a_timed_mover_forfeits_only_by_overrunning_the_clock_and_a_node_limited_mover_never_does() {
        assert!(!Thinking::Timed.forfeits(REMAINING, REMAINING));
        assert!(Thinking::Timed.forfeits(REMAINING + OVERRUN, REMAINING));
        assert!(!Thinking::FixedNodes(NodeCount::ONE).forfeits(REMAINING + OVERRUN, REMAINING));
    }
}
