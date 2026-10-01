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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use board::Color;
    use uci::Clock;

    use super::TimeBudget;

    const COMFORTABLE: Clock = Clock::new(
        Duration::from_secs(60),
        Duration::from_secs(60),
        Duration::from_secs(1),
        Duration::ZERO,
    );
    const NEARLY_OUT: Clock = Clock::new(
        Duration::from_millis(20),
        Duration::from_millis(20),
        Duration::from_millis(50),
        Duration::ZERO,
    );
    const SHARE_PLUS_INCREMENT: Duration = Duration::from_secs(3);
    const SHARE_ALONE: Duration = Duration::from_secs(2);

    #[test]
    fn a_move_gets_a_thirtieth_of_its_clock_plus_its_increment_but_never_more_than_the_clock_holds()
    {
        assert_eq!(
            TimeBudget::allot(&COMFORTABLE, Color::White),
            SHARE_PLUS_INCREMENT
        );
        assert_eq!(TimeBudget::allot(&COMFORTABLE, Color::Black), SHARE_ALONE);
        assert_eq!(
            TimeBudget::allot(&NEARLY_OUT, Color::White),
            TimeBudget::LEAST
        );
    }
}
