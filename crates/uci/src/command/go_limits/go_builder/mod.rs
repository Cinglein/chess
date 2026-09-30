use core::time::Duration;

use board::NodeCount;
use search::Depth;

use super::GoLimits;
use super::clock::Clock;
use super::go_key::GoKey;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct GoBuilder {
    pending: Option<GoKey>,
    depth: Option<Depth>,
    nodes: Option<NodeCount>,
    move_time: Option<Duration>,
    white_time: Option<Duration>,
    black_time: Option<Duration>,
    white_increment: Option<Duration>,
    black_increment: Option<Duration>,
}

impl GoBuilder {
    pub(super) fn absorb(self, token: &str) -> GoBuilder {
        match (self.pending, token.parse::<GoKey>()) {
            (Some(key), _) => self.assign(key, token),
            (None, Ok(GoKey::Infinite) | Err(_)) => self,
            (None, Ok(key)) => GoBuilder {
                pending: Some(key),
                ..self
            },
        }
    }

    pub(super) fn limits(self) -> GoLimits {
        self.depth
            .map(GoLimits::Depth)
            .or_else(|| self.nodes.map(GoLimits::Nodes))
            .or_else(|| self.move_time.map(GoLimits::MoveTime))
            .or_else(|| {
                self.white_time
                    .map(|white| GoLimits::Clock(self.clock(white)))
            })
            .unwrap_or(GoLimits::Infinite)
    }

    fn assign(self, key: GoKey, token: &str) -> GoBuilder {
        let cleared = GoBuilder {
            pending: None,
            ..self
        };
        match key {
            GoKey::Infinite => cleared,
            GoKey::Depth => GoBuilder {
                depth: token.parse().ok(),
                ..cleared
            },
            GoKey::Nodes => GoBuilder {
                nodes: token.parse().ok(),
                ..cleared
            },
            GoKey::MoveTime => GoBuilder {
                move_time: Self::milliseconds(token),
                ..cleared
            },
            GoKey::WTime => GoBuilder {
                white_time: Self::milliseconds(token),
                ..cleared
            },
            GoKey::BTime => GoBuilder {
                black_time: Self::milliseconds(token),
                ..cleared
            },
            GoKey::WInc => GoBuilder {
                white_increment: Self::milliseconds(token),
                ..cleared
            },
            GoKey::BInc => GoBuilder {
                black_increment: Self::milliseconds(token),
                ..cleared
            },
        }
    }

    fn milliseconds(token: &str) -> Option<Duration> {
        token.parse::<u64>().ok().map(Duration::from_millis)
    }

    fn clock(self, white: Duration) -> Clock {
        Clock::new(
            white,
            self.black_time.unwrap_or(white),
            self.white_increment.unwrap_or_default(),
            self.black_increment.unwrap_or_default(),
        )
    }
}
