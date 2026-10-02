mod thread_count;

use bullet_lib::trainer::settings::LocalSettings;
use serde::{Deserialize, Serialize};
use thread_count::ThreadCount;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    #[serde(default)]
    threads: ThreadCount,
    batch_queue_size: usize,
    output_directory: String,
}

impl Machine {
    #[must_use]
    pub fn local_settings(&self) -> LocalSettings<'_> {
        LocalSettings {
            threads: self.threads.count(),
            test_set: None,
            output_directory: &self.output_directory,
            batch_queue_size: self.batch_queue_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;

    const WRITTEN: &str = "
        threads = 3
        batch_queue_size = 4
        output_directory = \"nets\"
    ";

    #[test]
    fn machine_settings_become_bullets_local_settings_without_a_test_set() {
        let machine: Machine = toml::from_str(WRITTEN).unwrap();
        let local = machine.local_settings();
        assert_eq!(
            (
                local.threads,
                local.batch_queue_size,
                local.output_directory
            ),
            (3, 4, "nets")
        );
        assert!(local.test_set.is_none());
    }
}
