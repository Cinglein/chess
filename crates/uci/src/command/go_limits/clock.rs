use core::fmt;
use core::time::Duration;

use board::Color;

use super::go_key::GoKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock {
    white: Duration,
    black: Duration,
    white_increment: Duration,
    black_increment: Duration,
}

impl Clock {
    #[must_use]
    pub const fn new(
        white: Duration,
        black: Duration,
        white_increment: Duration,
        black_increment: Duration,
    ) -> Clock {
        Clock {
            white,
            black,
            white_increment,
            black_increment,
        }
    }

    #[must_use]
    pub const fn remaining(&self, side: Color) -> Duration {
        match side {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }

    #[must_use]
    pub const fn increment(&self, side: Color) -> Duration {
        match side {
            Color::White => self.white_increment,
            Color::Black => self.black_increment,
        }
    }

    #[must_use]
    pub fn minus_spent_plus_increment(self, side: Color, spent: Duration) -> Clock {
        match side {
            Color::White => Clock {
                white: self.white.saturating_sub(spent) + self.white_increment,
                ..self
            },
            Color::Black => Clock {
                black: self.black.saturating_sub(spent) + self.black_increment,
                ..self
            },
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {} {} {} {} {} {} {}",
            GoKey::WTime,
            self.white.as_millis(),
            GoKey::BTime,
            self.black.as_millis(),
            GoKey::WInc,
            self.white_increment.as_millis(),
            GoKey::BInc,
            self.black_increment.as_millis()
        )
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use board::Color;

    use super::Clock;

    const MINUTE: Duration = Duration::from_secs(60);
    const INCREMENT: Duration = Duration::from_secs(4);
    const SPENT: Duration = Duration::from_secs(2);
    const LEFT: Duration = Duration::from_secs(62);

    #[test]
    fn a_side_that_moved_loses_what_it_spent_and_gains_its_increment() {
        let clock = Clock::new(MINUTE, MINUTE, INCREMENT, Duration::ZERO)
            .minus_spent_plus_increment(Color::White, SPENT);
        assert_eq!(clock.remaining(Color::White), LEFT);
        assert_eq!(clock.remaining(Color::Black), MINUTE);
    }
}
