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
