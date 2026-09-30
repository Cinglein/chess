mod settings_error;
mod time_control;

pub use settings_error::SettingsError;

use std::fs;
use std::io;
use std::path::Path;

use arena::{DrawAdjudication, ResignAdjudication, RoundCount, Rules, WorkerCount};
use board::FullmoveNumber;
use serde::{Deserialize, Serialize};
use time_control::TimeControl;
use uci::{Clock, Elo};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

impl Settings {
    const DEFAULTS: &str = include_str!("arena.default.toml");

    pub fn read_or_write_defaults(path: &Path) -> Result<Settings, SettingsError> {
        match fs::read_to_string(path) {
            Ok(text) => Ok(toml::from_str(&text)?),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let settings = Settings::default();
                fs::write(path, toml::to_string_pretty(&settings)?)
                    .map_err(SettingsError::Write)?;
                Ok(settings)
            }
            Err(error) => Err(SettingsError::Read(error)),
        }
    }

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

impl Default for Settings {
    fn default() -> Settings {
        toml::from_str(Self::DEFAULTS).expect("the embedded default settings parse")
    }
}

#[cfg(test)]
mod tests {
    use arena::Rules;

    use super::Settings;

    #[test]
    fn default_settings_survive_a_trip_through_toml_and_agree_with_the_library_rules() {
        let written = toml::to_string_pretty(&Settings::default()).unwrap();
        let read: Settings = toml::from_str(&written).unwrap();
        assert_eq!(read, Settings::default());
        assert_eq!(read.rules(), Rules::DEFAULT);
    }
}
