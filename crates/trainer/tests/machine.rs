use trainer::Machine;
const WRITTEN: &str = "
    threads = 3
    batch_queue_size = 4
    output_directory = \"nets\"
";

#[test]
fn machine_settings_become_bullets_local_settings_without_a_test_set() {
    let machine: Machine = toml::from_str(WRITTEN).unwrap();
    let local = machine.local_settings();
    assert_eq!((local.threads, local.batch_queue_size), (3, 4));
    assert!(local.output_directory == "nets" && local.test_set.is_none());
}
