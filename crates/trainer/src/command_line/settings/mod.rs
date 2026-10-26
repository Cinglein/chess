use crate::{Machine, Schedule};
use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    net_id: String,
    schedule: Schedule,
    machine: Machine,
}

impl EmbeddedDefaults for Settings {
    const DEFAULTS: &str = include_str!("trainer.default.toml");
}

impl Settings {
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

#[cfg(test)]
mod tests {
    use super::Settings;
    use settings::EmbeddedDefaults;
    #[test]
    fn default_settings_survive_a_trip_through_toml() {
        let written = toml::to_string_pretty(&Settings::defaults()).unwrap();
        let read: Settings = toml::from_str(&written).unwrap();
        assert_eq!(read, Settings::defaults());
    }
}
