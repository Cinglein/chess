use arena::Settings;
use game::Rules;
use settings::EmbeddedDefaults;
#[test]
fn default_settings_agree_with_the_library_rules() {
    let read = Settings::defaults();
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
