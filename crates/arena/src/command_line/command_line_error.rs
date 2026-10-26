use crate::ArenaError;
use thiserror::Error;

use settings::SettingsError;

#[derive(Debug, Error)]
pub enum CommandLineError {
    #[error(transparent)]
    Settings(#[from] SettingsError),
    #[error(transparent)]
    Arena(#[from] ArenaError),
}
