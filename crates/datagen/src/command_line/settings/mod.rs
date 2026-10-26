use crate::PositionCount;
use arena::WorkerCount;
use board::{NodeCount, PlyCount};
use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    #[serde(default = "WorkerCount::one_per_core")]
    concurrency: WorkerCount,
    nodes_per_move: NodeCount,
    opening_plies: PlyCount,
    positions: PositionCount,
}

impl EmbeddedDefaults for Settings {
    const DEFAULTS: &str = include_str!("datagen.default.toml");
}

impl Settings {
    pub const fn concurrency(&self) -> WorkerCount {
        self.concurrency
    }

    pub const fn nodes_per_move(&self) -> NodeCount {
        self.nodes_per_move
    }

    pub const fn opening_plies(&self) -> PlyCount {
        self.opening_plies
    }

    pub const fn positions(&self) -> PositionCount {
        self.positions
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
