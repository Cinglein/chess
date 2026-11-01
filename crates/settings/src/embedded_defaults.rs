use std::fs;
use std::io;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::settings_error::SettingsError;

pub trait EmbeddedDefaults: DeserializeOwned {
    const DEFAULTS: &str;

    #[must_use]
    fn defaults() -> Self {
        toml::from_str(Self::DEFAULTS).expect("the embedded default settings parse")
    }

    fn read_or_write_defaults(path: &Path) -> Result<Self, SettingsError> {
        match fs::read_to_string(path) {
            Ok(text) => Ok(toml::from_str(&text)?),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let settings = Self::defaults();
                fs::write(path, Self::DEFAULTS).map_err(SettingsError::Write)?;
                Ok(settings)
            }
            Err(error) => Err(SettingsError::Read(error)),
        }
    }
}
