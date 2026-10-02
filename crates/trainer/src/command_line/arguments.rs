use std::path::{Path, PathBuf};

use clap::Parser;
use trainer::ShuffleSeed;

#[derive(Debug, Parser)]
#[command(about = "Shuffle a bullet data file and train the network on it with bullet")]
pub struct Arguments {
    #[arg(help = "Data file of bullet ChessBoard records, as datagen writes it")]
    records: PathBuf,
    #[arg(
        long,
        default_value_t = ShuffleSeed::default(),
        help = "Seed for the shuffle of the data file"
    )]
    seed: ShuffleSeed,
    #[arg(
        long,
        default_value = "trainer.toml",
        help = "Settings file; written with defaults when missing"
    )]
    config: PathBuf,
}

impl Arguments {
    pub fn records(&self) -> &Path {
        &self.records
    }

    pub const fn seed(&self) -> ShuffleSeed {
        self.seed
    }

    pub fn config(&self) -> &Path {
        &self.config
    }
}
