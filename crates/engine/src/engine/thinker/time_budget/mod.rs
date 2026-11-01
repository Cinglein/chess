mod moves_to_go;

use std::time::Duration;

use board::Color;
use moves_to_go::MovesToGo;
use uci::Clock;
use uci::{Increment, Remaining};

pub(super) struct TimeBudget;

impl TimeBudget {
    const MOVES_TO_GO: MovesToGo = MovesToGo::new(30);
    const SAFETY_MARGIN: Duration = Duration::from_millis(50);
    const LEAST: Duration = Duration::from_millis(1);

    pub(super) fn is_past_halfway(elapsed: Duration, allowance: Duration) -> bool {
        elapsed + elapsed >= allowance
    }

    pub(super) fn allot(clock: &Clock, side: Color) -> Duration {
        let remaining = clock.time::<Remaining>(side);
        (Self::MOVES_TO_GO.share_of(remaining) + clock.time::<Increment>(side))
            .min(remaining.saturating_sub(Self::SAFETY_MARGIN))
            .max(Self::LEAST)
    }
}
