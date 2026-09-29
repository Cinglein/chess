use std::time::Duration;

use uci::Clock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rules {
    clock: Clock,
    longest_game_plies: u16,
}

impl Rules {
    pub const DEFAULT: Rules = Rules {
        clock: Clock::new(
            Duration::from_secs(10),
            Duration::from_secs(10),
            Duration::from_millis(100),
            Duration::from_millis(100),
        ),
        longest_game_plies: 1000,
    };

    #[must_use]
    pub const fn timed(self, clock: Clock) -> Rules {
        Rules { clock, ..self }
    }

    #[must_use]
    pub const fn lasting_at_most(self, longest_game_plies: u16) -> Rules {
        Rules {
            longest_game_plies,
            ..self
        }
    }

    #[must_use]
    pub const fn clock(&self) -> Clock {
        self.clock
    }

    #[must_use]
    pub const fn longest_game_plies(&self) -> u16 {
        self.longest_game_plies
    }
}
