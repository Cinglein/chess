use settings::EmbeddedDefaults;
use trainer::Settings;
use trainer_schedule::NetId;
#[test]
fn the_embedded_defaults_parse_and_name_the_net() {
    assert_eq!(
        Settings::defaults().net_id(),
        &NetId::try_from("chess".to_owned()).unwrap()
    );
}
