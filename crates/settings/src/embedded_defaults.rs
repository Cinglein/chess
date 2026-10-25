use std::fs;
use std::io;
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::settings_error::SettingsError;

pub trait EmbeddedDefaults: Serialize + DeserializeOwned {
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
                fs::write(path, toml::to_string_pretty(&settings)?)
                    .map_err(SettingsError::Write)?;
                Ok(settings)
            }
            Err(error) => Err(SettingsError::Read(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde::{Deserialize, Serialize};

    use super::EmbeddedDefaults;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Knobs {
        rounds: usize,
    }

    impl EmbeddedDefaults for Knobs {
        const DEFAULTS: &str = "rounds = 3";
    }

    #[test]
    fn a_missing_file_is_written_with_the_defaults_and_read_back_while_an_unknown_key_is_refused() {
        let path = std::env::temp_dir().join("knobs.toml");
        fs::remove_file(&path).ok();
        let written = Knobs::read_or_write_defaults(&path).unwrap();
        let read = Knobs::read_or_write_defaults(&path).unwrap();
        assert_eq!((written, read), (Knobs::defaults(), Knobs { rounds: 3 }));
        fs::write(&path, "rounds = 3\nrunds = 4").unwrap();
        assert!(Knobs::read_or_write_defaults(&path).is_err());
    }
}
