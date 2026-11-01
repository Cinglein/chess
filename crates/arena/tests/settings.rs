use arena::Settings;
use game::Rules;
use settings::EmbeddedDefaults;
#[test]
fn default_settings_survive_a_trip_through_toml_and_agree_with_the_library_rules() {
    let written = toml::to_string_pretty(&Settings::defaults()).unwrap();
    let read: Settings = toml::from_str(&written).unwrap();
    assert_eq!(read, Settings::defaults());
    assert_eq!(
        Rules::new(
            read.time_control(),
            read.longest_game(),
            read.draw_adjudication(),
            read.resign_adjudication()
        ),
        Rules::DEFAULT
    );
}
