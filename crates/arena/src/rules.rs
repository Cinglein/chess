use std::num::NonZeroU16;
use std::time::Duration;

use board::FullmoveNumber;
use uci::Clock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rules {
    clock: Clock,
    longest_game: FullmoveNumber,
}

impl Rules {
    pub const DEFAULT: Rules = Rules {
        clock: Clock::new(
            Duration::from_secs(10),
            Duration::from_secs(10),
            Duration::from_millis(100),
            Duration::from_millis(100),
        ),
        longest_game: FullmoveNumber::new(NonZeroU16::new(500).unwrap()),
    };

    #[must_use]
    pub const fn timed(self, clock: Clock) -> Rules {
        Rules { clock, ..self }
    }

    #[must_use]
    pub const fn lasting_at_most(self, longest_game: FullmoveNumber) -> Rules {
        Rules {
            longest_game,
            ..self
        }
    }

    #[must_use]
    pub const fn clock(&self) -> Clock {
        self.clock
    }

    #[must_use]
    pub const fn longest_game(&self) -> FullmoveNumber {
        self.longest_game
    }
}
