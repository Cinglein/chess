use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("could not read the settings file: {0}")]
    Read(io::Error),
    #[error("could not write the settings file: {0}")]
    Write(io::Error),
    #[error("the settings file is malformed: {0}")]
    Format(#[from] toml::de::Error),
    #[error("the default settings could not be written out: {0}")]
    Serialize(#[from] toml::ser::Error),
}
