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

#[cfg(test)]
mod tests {
    use super::TimeBudget;
    use board::Color;
    use std::time::Duration;
    use uci::Clock;
    const MINUTE: Duration = Duration::from_secs(60);
    const MOMENT: Duration = Duration::from_millis(20);
    const COMFORTABLE: Clock = Clock::new(MINUTE, MINUTE, Duration::from_secs(1), Duration::ZERO);
    const NEARLY_OUT: Clock = Clock::new(MOMENT, MOMENT, TimeBudget::SAFETY_MARGIN, Duration::ZERO);
    const ALLOTTED: [(Clock, Color, Duration); 3] = [
        (COMFORTABLE, Color::White, Duration::from_secs(3)),
        (COMFORTABLE, Color::Black, Duration::from_secs(2)),
        (NEARLY_OUT, Color::White, TimeBudget::LEAST),
    ];
    const ALLOWANCE: Duration = Duration::from_secs(4);
    const HALF: Duration = Duration::from_secs(2);
    const JUST_UNDER_HALF: Duration = HALF.saturating_sub(Duration::from_nanos(1));

    #[test]
    fn a_move_gets_a_thirtieth_of_its_clock_plus_its_increment_but_never_more_than_the_clock_holds()
    {
        for (clock, side, allotted) in ALLOTTED {
            assert_eq!(TimeBudget::allot(&clock, side), allotted, "{side:?}");
        }
    }

    #[test]
    fn deepening_stops_once_half_the_allowance_is_spent() {
        assert!(!TimeBudget::is_past_halfway(JUST_UNDER_HALF, ALLOWANCE));
        assert!(TimeBudget::is_past_halfway(HALF, ALLOWANCE));
    }
}
