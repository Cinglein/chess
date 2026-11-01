use std::fmt;
use std::time::{Duration, Instant};

use crate::position_count::PositionCount;

#[derive(Clone, Copy, Debug)]
pub struct Progress {
    written: PositionCount,
    target: PositionCount,
    started: Instant,
    reported: Instant,
}

impl Progress {
    const INTERVAL: Duration = Duration::from_secs(5);

    #[must_use]
    pub fn towards(target: PositionCount) -> Progress {
        let now = Instant::now();
        Progress {
            written: PositionCount::default(),
            target,
            started: now,
            reported: now,
        }
    }

    #[must_use]
    pub fn advanced(self, by: PositionCount) -> Progress {
        Progress {
            written: self.written + by,
            ..self
        }
    }

    #[must_use]
    pub fn is_due(&self, now: Instant) -> bool {
        now.saturating_duration_since(self.reported) >= Self::INTERVAL
    }

    #[must_use]
    pub fn acknowledged(self, now: Instant) -> Progress {
        Progress {
            reported: now,
            ..self
        }
    }
}

impl fmt::Display for Progress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}/{} positions, {} per second",
            self.written,
            self.target,
            self.written.per_second(self.started.elapsed())
        )
    }
}
