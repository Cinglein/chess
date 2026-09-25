use core::time::Duration;

use search::Depth;

use super::GoLimits;
use super::clock::Clock;
use super::go_key::GoKey;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct GoBuilder {
    pending: Option<GoKey>,
    depth: Option<u64>,
    nodes: Option<u64>,
    move_time: Option<u64>,
    white_time: Option<u64>,
    black_time: Option<u64>,
    white_increment: Option<u64>,
    black_increment: Option<u64>,
}

impl GoBuilder {
    pub(super) fn absorb(self, token: &str) -> GoBuilder {
        match (self.pending, token.parse::<GoKey>()) {
            (Some(key), _) => self.assign(key, token.parse().ok()),
            (None, Ok(GoKey::Infinite) | Err(_)) => self,
            (None, Ok(key)) => GoBuilder {
                pending: Some(key),
                ..self
            },
        }
    }

    pub(super) fn limits(self) -> GoLimits {
        self.depth
            .and_then(|plies| u8::try_from(plies).ok())
            .map(|plies| GoLimits::Depth(Depth::new(plies)))
            .or_else(|| self.nodes.map(GoLimits::Nodes))
            .or_else(|| {
                self.move_time
                    .map(|millis| GoLimits::MoveTime(Duration::from_millis(millis)))
            })
            .or_else(|| {
                self.white_time
                    .map(|white| GoLimits::Clock(self.clock(white)))
            })
            .unwrap_or(GoLimits::Infinite)
    }

    fn assign(self, key: GoKey, value: Option<u64>) -> GoBuilder {
        let cleared = GoBuilder {
            pending: None,
            ..self
        };
        match key {
            GoKey::Infinite => cleared,
            GoKey::Depth => GoBuilder {
                depth: value,
                ..cleared
            },
            GoKey::Nodes => GoBuilder {
                nodes: value,
                ..cleared
            },
            GoKey::MoveTime => GoBuilder {
                move_time: value,
                ..cleared
            },
            GoKey::WTime => GoBuilder {
                white_time: value,
                ..cleared
            },
            GoKey::BTime => GoBuilder {
                black_time: value,
                ..cleared
            },
            GoKey::WInc => GoBuilder {
                white_increment: value,
                ..cleared
            },
            GoKey::BInc => GoBuilder {
                black_increment: value,
                ..cleared
            },
        }
    }

    fn clock(self, white: u64) -> Clock {
        Clock::new(
            Duration::from_millis(white),
            Duration::from_millis(self.black_time.unwrap_or(white)),
            Duration::from_millis(self.white_increment.unwrap_or(0)),
            Duration::from_millis(self.black_increment.unwrap_or(0)),
        )
    }
}
