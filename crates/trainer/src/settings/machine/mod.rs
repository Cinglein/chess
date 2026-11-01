mod output_directory;
mod thread_count;

use bullet_lib::trainer::settings::LocalSettings;
use output_directory::OutputDirectory;
use serde::Deserialize;
use thread_count::ThreadCount;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Machine {
    #[serde(default)]
    threads: ThreadCount,
    batch_queue_size: usize,
    output_directory: OutputDirectory,
}

impl Machine {
    #[must_use]
    pub fn local_settings(&self) -> LocalSettings<'_> {
        LocalSettings {
            threads: usize::from(self.threads),
            test_set: None,
            output_directory: self.output_directory.as_ref(),
            batch_queue_size: self.batch_queue_size,
        }
    }
}
