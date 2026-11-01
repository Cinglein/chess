use crate::Machine;
use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;
use trainer_schedule::Schedule;

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
    #[must_use]
    pub fn net_id(&self) -> &str {
        &self.net_id
    }

    #[must_use]
    pub const fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    #[must_use]
    pub const fn machine(&self) -> &Machine {
        &self.machine
    }
}
