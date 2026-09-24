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
