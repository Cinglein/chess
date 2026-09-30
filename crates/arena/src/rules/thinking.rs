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
