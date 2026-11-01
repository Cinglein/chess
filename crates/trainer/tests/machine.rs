use settings::EmbeddedDefaults;
use trainer::Settings;

#[test]
fn machine_settings_become_bullets_local_settings_without_a_test_set() {
    let settings = Settings::defaults();
    let local = settings.machine().local_settings();
    assert_eq!((local.threads, local.batch_queue_size), (8, 64));
    assert!(local.output_directory == "checkpoints" && local.test_set.is_none());
}

#[test]
fn an_empty_output_directory_is_refused() {
    let written = Settings::DEFAULTS.replace("\"checkpoints\"", "\"\"");
    assert!(toml::from_str::<Settings>(&written).is_err());
}
