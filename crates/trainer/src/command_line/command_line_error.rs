use std::io;

use eval::SavedNetworkError;
use thiserror::Error;
use trainer::TrainerError;

use settings::SettingsError;

#[derive(Debug, Error)]
pub enum CommandLineError {
    #[error(transparent)]
    Settings(#[from] SettingsError),
    #[error(transparent)]
    Trainer(#[from] TrainerError),
    #[error("the data path is not valid text")]
    Path,
    #[error("could not read the quantised network: {0}")]
    Checkpoint(io::Error),
    #[error(transparent)]
    Network(#[from] SavedNetworkError),
}
