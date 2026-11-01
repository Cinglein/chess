use datagen::Settings;
use settings::EmbeddedDefaults;
#[test]
fn the_embedded_defaults_parse_and_ask_for_positions() {
    assert!(Settings::defaults().positions().count() > 0);
}
