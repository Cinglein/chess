mod machine;

use ::settings::EmbeddedDefaults;
use machine::Machine;
use serde::Deserialize;
use trainer_schedule::{NetId, Schedule};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    net_id: NetId,
    schedule: Schedule,
    machine: Machine,
}

impl EmbeddedDefaults for Settings {
    const DEFAULTS: &str = include_str!("trainer.default.toml");
}

impl Settings {
    #[must_use]
    pub const fn net_id(&self) -> &NetId {
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
