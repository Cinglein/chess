mod contestant;

pub use contestant::Contestant;

use std::path::{Path, PathBuf};

use clap::Parser;

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
    pub const fn challenger(&self) -> &Contestant {
        &self.challenger
    }

    pub const fn reference(&self) -> &Contestant {
        &self.reference
    }

    pub fn config(&self) -> &Path {
        &self.config
    }
}
