mod draw_adjudication;
mod resign_adjudication;
mod thinking;

pub use draw_adjudication::DrawAdjudication;
pub use resign_adjudication::ResignAdjudication;
pub use thinking::Thinking;

use std::num::NonZeroU16;
use std::time::Duration;

use board::{FullmoveNumber, PlyCount};
use eval::Score;
use uci::Clock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rules {
    clock: Clock,
    thinking: Thinking,
    longest_game: FullmoveNumber,
    draw: DrawAdjudication,
    resign: ResignAdjudication,
}

impl Rules {
    pub const DEFAULT: Rules = Rules {
        thinking: Thinking::Timed,
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
            PlyCount::new(8),
        ),
        resign: ResignAdjudication::new(Score::new(800), PlyCount::new(6)),
    };

    #[must_use]
    pub const fn timed(self, clock: Clock) -> Rules {
        Rules { clock, ..self }
    }

    #[must_use]
    pub const fn thinking_by(self, thinking: Thinking) -> Rules {
        Rules { thinking, ..self }
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
    pub const fn thinking(&self) -> Thinking {
        self.thinking
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

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;
    use std::time::Duration;

    use board::{FullmoveNumber, NodeCount, PlyCount};
    use eval::Score;
    use uci::Clock;

    use super::{DrawAdjudication, ResignAdjudication, Rules, Thinking};

    const CLOCK: Clock = Clock::new(
        Duration::from_secs(1),
        Duration::from_secs(2),
        Duration::ZERO,
        Duration::ZERO,
    );
    const THINKING: Thinking = Thinking::FixedNodes(NodeCount::new(64));
    const LONGEST: FullmoveNumber = FullmoveNumber::new(NonZeroU16::MIN);
    const DRAW: DrawAdjudication = DrawAdjudication::new(LONGEST, Score::DRAW, PlyCount::ZERO);
    const RESIGN: ResignAdjudication = ResignAdjudication::new(Score::DRAW, PlyCount::ZERO);

    #[test]
    fn every_rule_a_builder_sets_is_read_back_by_its_accessor() {
        let rules = Rules::DEFAULT
            .timed(CLOCK)
            .thinking_by(THINKING)
            .lasting_at_most(LONGEST)
            .adjudicated_by(DRAW, RESIGN);
        assert_eq!(
            (
                rules.clock(),
                rules.thinking(),
                rules.longest_game(),
                rules.draw(),
                rules.resign()
            ),
            (CLOCK, THINKING, LONGEST, DRAW, RESIGN)
        );
    }
}
