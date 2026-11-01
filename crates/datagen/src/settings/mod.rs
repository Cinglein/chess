use crate::PositionCount;
use ::settings::EmbeddedDefaults;
use arena::WorkerCount;
use board::{NodeCount, PlyCount};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
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
    #[must_use]
    pub const fn concurrency(&self) -> WorkerCount {
        self.concurrency
    }

    #[must_use]
    pub const fn nodes_per_move(&self) -> NodeCount {
        self.nodes_per_move
    }

    #[must_use]
    pub const fn opening_plies(&self) -> PlyCount {
        self.opening_plies
    }

    #[must_use]
    pub const fn positions(&self) -> PositionCount {
        self.positions
    }
}
