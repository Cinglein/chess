mod clock;
mod go_builder;

pub use clock::Clock;

use core::fmt;
use core::time::Duration;

use go_builder::GoBuilder;
use search::Depth;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoLimits {
    Infinite,
    Depth(Depth),
    Nodes(u64),
    MoveTime(Duration),
    Clock(Clock),
}

impl GoLimits {
    #[must_use]
    pub const fn depth(&self) -> Option<Depth> {
        match self {
            GoLimits::Depth(depth) => Some(*depth),
            _ => None,
        }
    }

    #[must_use]
    pub const fn nodes(&self) -> Option<u64> {
        match self {
            GoLimits::Nodes(nodes) => Some(*nodes),
            _ => None,
        }
    }

    #[must_use]
    pub const fn move_time(&self) -> Option<Duration> {
        match self {
            GoLimits::MoveTime(duration) => Some(*duration),
            _ => None,
        }
    }

    #[must_use]
    pub const fn clock(&self) -> Option<Clock> {
        match self {
            GoLimits::Clock(clock) => Some(*clock),
            _ => None,
        }
    }
}

impl From<&str> for GoLimits {
    fn from(rest: &str) -> GoLimits {
        rest.split_whitespace()
            .fold(GoBuilder::default(), GoBuilder::absorb)
            .limits()
    }
}

impl fmt::Display for GoLimits {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoLimits::Infinite => formatter.write_str("infinite"),
            GoLimits::Depth(depth) => write!(formatter, "depth {depth}"),
            GoLimits::Nodes(nodes) => write!(formatter, "nodes {nodes}"),
            GoLimits::MoveTime(duration) => write!(formatter, "movetime {}", duration.as_millis()),
            GoLimits::Clock(clock) => write!(formatter, "{clock}"),
        }
    }
}
