use core::fmt;
use std::path::{Path, PathBuf};

use super::Schedule;
use super::superbatch_count::SuperbatchCount;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checkpoint {
    net_id: String,
    superbatch: SuperbatchCount,
}

impl Checkpoint {
    const QUANTISED_FILE: &str = "quantised.bin";

    #[must_use]
    pub fn last_of(schedule: &Schedule, net_id: &str) -> Checkpoint {
        Checkpoint {
            net_id: net_id.to_owned(),
            superbatch: schedule.superbatches,
        }
    }

    #[must_use]
    pub fn quantised_path(&self, output_directory: &str) -> PathBuf {
        Path::new(output_directory)
            .join(self.to_string())
            .join(Self::QUANTISED_FILE)
    }
}

impl fmt::Display for Checkpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}-{}", self.net_id, self.superbatch)
    }
}

#[cfg(test)]
mod tests {
    use super::{Checkpoint, Path, Schedule};
    const WRITTEN: &str = "
        batch_size = 8
        batches_per_superbatch = 4
        superbatches = 6
        save_every = 3
        learning_rate = 0.5
        learning_rate_decay = 0.1
        decay_every = 2
        wdl = 0.75
    ";

    #[test]
    fn the_quantised_file_sits_under_the_output_directory_in_a_folder_named_after_the_net_and_last_superbatch()
     {
        let schedule: Schedule = toml::from_str(WRITTEN).unwrap();
        let checkpoint = Checkpoint::last_of(&schedule, "chess");
        assert_eq!(
            checkpoint.quantised_path("nets"),
            Path::new("nets/chess-6/quantised.bin")
        );
    }
}
