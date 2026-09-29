mod moves_to_go;

use std::time::Duration;

use board::Color;
use moves_to_go::MovesToGo;
use uci::Clock;

pub(super) struct TimeBudget;

impl TimeBudget {
    const MOVES_TO_GO: MovesToGo = MovesToGo::new(30);
    const SAFETY_MARGIN: Duration = Duration::from_millis(50);
    const LEAST: Duration = Duration::from_millis(1);

    pub(super) fn allot(clock: &Clock, side: Color) -> Duration {
        let remaining = clock.remaining(side);
        (Self::MOVES_TO_GO.share_of(remaining) + clock.increment(side))
            .min(remaining.saturating_sub(Self::SAFETY_MARGIN))
            .max(Self::LEAST)
    }
}
