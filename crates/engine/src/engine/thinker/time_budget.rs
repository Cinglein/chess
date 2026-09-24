use std::time::Duration;

use board::Color;
use uci::Clock;

pub(super) struct TimeBudget;

impl TimeBudget {
    const MOVES_TO_GO: u32 = 30;
    const SAFETY_MARGIN: Duration = Duration::from_millis(50);
    const LEAST: Duration = Duration::from_millis(1);

    pub(super) fn allot(clock: &Clock, side: Color) -> Duration {
        let remaining = clock.remaining(side);
        (remaining / Self::MOVES_TO_GO + clock.increment(side))
            .min(remaining.saturating_sub(Self::SAFETY_MARGIN))
            .max(Self::LEAST)
    }
}
