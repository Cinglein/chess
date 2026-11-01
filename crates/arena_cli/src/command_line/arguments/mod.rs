mod challenger;
mod reference;
mod role;

pub use challenger::Challenger;
pub use reference::Reference;
pub use role::Role;

use std::path::{Path, PathBuf};

use clap::Parser;
use opponent_ports::Contestant;

#[derive(Debug, Parser)]
#[command(about = "Play paired games from a set of openings and tally them for the challenger")]
pub struct Arguments {
    #[arg(help = "in-process, or a path to a UCI engine")]
    challenger: Contestant,
    #[arg(help = "in-process, or a path to a UCI engine")]
    reference: Contestant,
    #[arg(
        long,
        default_value = "arena.toml",
        help = "Settings file; written with defaults when missing"
    )]
    config: PathBuf,
}

impl Arguments {
    pub fn contestant<R: Role>(&self) -> &Contestant {
        R::pick(self)
    }

    pub fn config(&self) -> &Path {
        &self.config
    }
}
