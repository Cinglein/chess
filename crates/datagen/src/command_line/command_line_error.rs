use std::io;

use crate::DatagenError;
use thiserror::Error;

use settings::SettingsError;

#[derive(Debug, Error)]
pub enum CommandLineError {
    #[error(transparent)]
    Settings(#[from] SettingsError),
    #[error(transparent)]
    Datagen(#[from] DatagenError),
    #[error("could not write the output file: {0}")]
    Output(io::Error),
}
