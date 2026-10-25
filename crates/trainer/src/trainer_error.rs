use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrainerError {
    #[error("could not read the data file: {0}")]
    Read(io::Error),
    #[error("could not write the shuffled data file: {0}")]
    Write(io::Error),
    #[error("the data file does not end on a record boundary")]
    Truncated,
}
