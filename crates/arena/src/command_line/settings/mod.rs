use crate::{DrawAdjudication, ResignAdjudication, RoundCount, TimeControl, WorkerCount};
use board::FullmoveNumber;
use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;
use uci::Elo;

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

    pub const fn longest_game(&self) -> FullmoveNumber {
        self.longest_game
    }

    pub const fn time_control(&self) -> TimeControl {
        self.time_control
    }

    pub const fn draw_adjudication(&self) -> DrawAdjudication {
        self.draw_adjudication
    }

    pub const fn resign_adjudication(&self) -> ResignAdjudication {
        self.resign_adjudication
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;
    use crate::Rules;
    use settings::EmbeddedDefaults;
    #[test]
    fn default_settings_survive_a_trip_through_toml_and_agree_with_the_library_rules() {
        let written = toml::to_string_pretty(&Settings::defaults()).unwrap();
        let read: Settings = toml::from_str(&written).unwrap();
        assert_eq!(read, Settings::defaults());
        assert_eq!(
            Rules::new(
                read.time_control(),
                read.longest_game(),
                read.draw_adjudication(),
                read.resign_adjudication()
            ),
            Rules::DEFAULT
        );
    }
}
