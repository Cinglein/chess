mod settings_error;

pub use settings_error::SettingsError;

use std::fs;
use std::io;
use std::path::Path;

use arena::{Rules, Thinking, WorkerCount};
use board::{NodeCount, PlyCount};
use datagen::PositionCount;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "WorkerCount::one_per_core")]
    concurrency: WorkerCount,
    nodes_per_move: NodeCount,
    opening_plies: PlyCount,
    positions: PositionCount,
}

impl Settings {
    const DEFAULTS: &str = include_str!("datagen.default.toml");

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

    pub const fn opening_plies(&self) -> PlyCount {
        self.opening_plies
    }

    pub const fn positions(&self) -> PositionCount {
        self.positions
    }

    pub const fn rules(&self) -> Rules {
        Rules::DEFAULT.thinking_by(Thinking::FixedNodes(self.nodes_per_move))
    }
}

impl Default for Settings {
    fn default() -> Settings {
        toml::from_str(Self::DEFAULTS).expect("the embedded default settings parse")
    }
}

#[cfg(test)]
mod tests {
    use arena::Thinking;

    use super::Settings;

    #[test]
    fn default_settings_survive_a_trip_through_toml_and_limit_every_move_by_nodes() {
        let written = toml::to_string_pretty(&Settings::default()).unwrap();
        let read: Settings = toml::from_str(&written).unwrap();
        assert_eq!(read, Settings::default());
        assert!(matches!(read.rules().thinking(), Thinking::FixedNodes(_)));
    }
}
