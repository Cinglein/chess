mod thread_count;

use bullet_lib::trainer::settings::LocalSettings;
use serde::{Deserialize, Serialize};
use thread_count::ThreadCount;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
