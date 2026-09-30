use arena::ArenaError;
use thiserror::Error;

use super::settings::SettingsError;

#[derive(Debug, Error)]
pub enum CommandLineError {
    #[error(transparent)]
    Settings(#[from] SettingsError),
    #[error(transparent)]
    Arena(#[from] ArenaError),
}
