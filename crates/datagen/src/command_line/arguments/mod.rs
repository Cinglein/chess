use std::path::{Path, PathBuf};

use arena::Seed;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    about = "Play the engine against itself from random openings and write bullet training records"
)]
pub struct Arguments {
    #[arg(help = "Output file of bullet ChessBoard records")]
    output: PathBuf,
    #[arg(
        long,
        default_value_t = Seed::from_clock(),
        help = "Seed for the random openings; taken from the clock when omitted"
    )]
    seed: Seed,
    #[arg(
        long,
        default_value = "datagen.toml",
        help = "Settings file; written with defaults when missing"
    )]
    config: PathBuf,
}

impl Arguments {
    pub fn output(&self) -> &Path {
        &self.output
    }

    pub const fn seed(&self) -> Seed {
        self.seed
    }

    pub fn config(&self) -> &Path {
        &self.config
    }
}
