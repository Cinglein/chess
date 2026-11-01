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
}
