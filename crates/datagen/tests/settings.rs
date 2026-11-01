use datagen::Settings;
use settings::EmbeddedDefaults;
#[test]
fn default_settings_survive_a_trip_through_toml() {
    let written = toml::to_string_pretty(&Settings::defaults()).unwrap();
    let read: Settings = toml::from_str(&written).unwrap();
    assert_eq!(read, Settings::defaults());
}
