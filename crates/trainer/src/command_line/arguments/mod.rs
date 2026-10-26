mod argument_file;
mod config;
mod records;

pub use argument_file::ArgumentFile;
pub use config::Config;
pub use records::Records;

use std::path::{Path, PathBuf};

use crate::ShuffleSeed;
use clap::Parser;

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
    pub fn path<F: ArgumentFile>(&self) -> &Path {
        F::pick(self)
    }

    pub const fn seed(&self) -> ShuffleSeed {
        self.seed
    }
}
