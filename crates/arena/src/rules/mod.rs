mod draw_adjudication;
mod resign_adjudication;

pub use draw_adjudication::DrawAdjudication;
pub use resign_adjudication::ResignAdjudication;

use std::num::NonZeroU16;
use std::time::Duration;

use board::FullmoveNumber;
use eval::Score;
use uci::Clock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rules {
    clock: Clock,
    longest_game: FullmoveNumber,
    draw: DrawAdjudication,
    resign: ResignAdjudication,
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
        draw: DrawAdjudication::new(
            FullmoveNumber::new(NonZeroU16::new(40).unwrap()),
            Score::new(10),
            8,
        ),
        resign: ResignAdjudication::new(Score::new(800), 6),
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
    pub const fn adjudicated_by(self, draw: DrawAdjudication, resign: ResignAdjudication) -> Rules {
        Rules {
            draw,
            resign,
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

    #[must_use]
    pub const fn draw(&self) -> DrawAdjudication {
        self.draw
    }

    #[must_use]
    pub const fn resign(&self) -> ResignAdjudication {
        self.resign
    }
}
