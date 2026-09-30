mod settings_error;
mod time_control;

pub use settings_error::SettingsError;

use std::fs;
use std::io;
use std::num::NonZeroUsize;
use std::path::Path;
use std::thread;

use arena::Rules;
use board::FullmoveNumber;
use serde::{Deserialize, Serialize};
use time_control::TimeControl;
use uci::{Clock, Elo};

use super::arguments::Contestant;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    concurrency: usize,
    rounds: usize,
    limit_elo: Elo,
    longest_game: FullmoveNumber,
    time_control: TimeControl,
}

impl Settings {
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

    pub const fn concurrency(&self) -> usize {
        self.concurrency
    }

    pub const fn rounds(&self) -> usize {
        self.rounds
    }

    pub const fn limit_elo(&self) -> Elo {
        self.limit_elo
    }

    pub fn rules(&self) -> Rules {
        Rules::DEFAULT
            .timed(Clock::from(self.time_control))
            .lasting_at_most(self.longest_game)
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            concurrency: thread::available_parallelism().map_or(1, NonZeroUsize::get),
            rounds: 1,
            limit_elo: Contestant::STOCKFISH_FLOOR,
            longest_game: Rules::DEFAULT.longest_game(),
            time_control: TimeControl::from(Rules::DEFAULT.clock()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;

    #[test]
    fn default_settings_survive_a_trip_through_toml() {
        let written = toml::to_string_pretty(&Settings::default()).unwrap();
        let read: Settings = toml::from_str(&written).unwrap();
        assert_eq!(read, Settings::default());
    }
}
