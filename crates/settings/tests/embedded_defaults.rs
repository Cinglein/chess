use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;
use settings::SettingsError;
use std::fs;
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
    assert!(matches!(
        Knobs::read_or_write_defaults(&std::env::temp_dir()),
        Err(SettingsError::Read(_))
    ));
}
