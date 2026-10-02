mod settings_error;

pub use settings_error::SettingsError;

use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};
use trainer::{Machine, Schedule};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    net_id: String,
    schedule: Schedule,
    machine: Machine,
}

impl Settings {
    const DEFAULTS: &str = include_str!("trainer.default.toml");

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

    pub fn net_id(&self) -> &str {
        &self.net_id
    }

    pub const fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub const fn machine(&self) -> &Machine {
        &self.machine
    }
}

impl Default for Settings {
    fn default() -> Settings {
        toml::from_str(Self::DEFAULTS).expect("the embedded default settings parse")
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
