mod clock;
mod go_builder;
mod go_key;

pub use clock::Clock;

use core::fmt;
use core::time::Duration;

use board::NodeCount;
use go_builder::GoBuilder;
use go_key::GoKey;
use search::Depth;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoLimits {
    Infinite,
    Depth(Depth),
    Nodes(NodeCount),
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
    pub const fn nodes(&self) -> Option<NodeCount> {
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
            GoLimits::Infinite => write!(formatter, "{}", GoKey::Infinite),
            GoLimits::Depth(depth) => write!(formatter, "{} {depth}", GoKey::Depth),
            GoLimits::Nodes(nodes) => write!(formatter, "{} {nodes}", GoKey::Nodes),
            GoLimits::MoveTime(duration) => {
                write!(formatter, "{} {}", GoKey::MoveTime, duration.as_millis())
            }
            GoLimits::Clock(clock) => write!(formatter, "{clock}"),
        }
    }
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for GoLimits {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<GoLimits>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::{Just, Strategy};
        proptest::prop_oneof![
            Just(GoLimits::Infinite),
            proptest::arbitrary::any::<Depth>().prop_map(GoLimits::Depth),
            proptest::arbitrary::any::<NodeCount>().prop_map(GoLimits::Nodes),
            proptest::arbitrary::any::<u32>()
                .prop_map(|millis| GoLimits::MoveTime(Duration::from_millis(u64::from(millis)))),
            proptest::arbitrary::any::<Clock>().prop_map(GoLimits::Clock),
        ]
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::GoLimits;

    #[test]
    fn limits_print_back_to_themselves_and_expose_exactly_their_own_kind() {
        proptest!(|(limits: GoLimits)| {
            prop_assert_eq!(GoLimits::from(limits.to_string().as_str()), limits);
            let exposed = [limits.depth().is_some(), limits.nodes().is_some(), limits.move_time().is_some(), limits.clock().is_some()];
            prop_assert_eq!(exposed.iter().filter(|kind| **kind).count(), usize::from(limits != GoLimits::Infinite));
        });
    }
}
