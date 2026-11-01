use crate::WorkerCount;
use crate::round_count::RoundCount;
use ::settings::EmbeddedDefaults;
use board::FullmoveNumber;
use game::{DrawAdjudication, ResignAdjudication, TimeControl};
use serde::Deserialize;
use uci::Elo;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    #[serde(default = "WorkerCount::one_per_core")]
    concurrency: WorkerCount,
    rounds: RoundCount,
    limit_elo: Elo,
    longest_game: FullmoveNumber,
    time_control: TimeControl,
    draw_adjudication: DrawAdjudication,
    resign_adjudication: ResignAdjudication,
}

impl EmbeddedDefaults for Settings {
    const DEFAULTS: &str = include_str!("arena.default.toml");
}

impl Settings {
    #[must_use]
    pub const fn concurrency(&self) -> WorkerCount {
        self.concurrency
    }

    #[must_use]
    pub const fn rounds(&self) -> RoundCount {
        self.rounds
    }

    #[must_use]
    pub const fn limit_elo(&self) -> Elo {
        self.limit_elo
    }

    #[must_use]
    pub const fn longest_game(&self) -> FullmoveNumber {
        self.longest_game
    }

    #[must_use]
    pub const fn time_control(&self) -> TimeControl {
        self.time_control
    }

    #[must_use]
    pub const fn draw_adjudication(&self) -> DrawAdjudication {
        self.draw_adjudication
    }

    #[must_use]
    pub const fn resign_adjudication(&self) -> ResignAdjudication {
        self.resign_adjudication
    }
}
