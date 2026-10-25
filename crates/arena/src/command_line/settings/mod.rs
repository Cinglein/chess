mod time_control;

use arena::{DrawAdjudication, ResignAdjudication, RoundCount, Rules, WorkerCount};
use board::FullmoveNumber;
use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;
use time_control::TimeControl;
use uci::{Clock, Elo};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub const fn concurrency(&self) -> WorkerCount {
        self.concurrency
    }

    pub const fn rounds(&self) -> RoundCount {
        self.rounds
    }

    pub const fn limit_elo(&self) -> Elo {
        self.limit_elo
    }

    pub fn rules(&self) -> Rules {
        Rules::DEFAULT
            .timed(Clock::from(self.time_control))
            .lasting_at_most(self.longest_game)
            .adjudicated_by(self.draw_adjudication, self.resign_adjudication)
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;
    use arena::Rules;
    use settings::EmbeddedDefaults;

    #[test]
    fn default_settings_survive_a_trip_through_toml_and_agree_with_the_library_rules() {
        let written = toml::to_string_pretty(&Settings::defaults()).unwrap();
        let read: Settings = toml::from_str(&written).unwrap();
        assert_eq!(read, Settings::defaults());
        assert_eq!(read.rules(), Rules::DEFAULT);
    }
}
