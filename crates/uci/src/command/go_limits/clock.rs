use core::time::Duration;

use board::Color;

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
