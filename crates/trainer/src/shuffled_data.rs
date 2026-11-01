use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use bulletformat::ChessBoard;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::shuffle_seed::ShuffleSeed;
use crate::trainer_error::TrainerError;

pub struct ShuffledData {
    path: PathBuf,
}

impl ShuffledData {
    const RECORD_BYTES: usize = size_of::<ChessBoard>();
    const EXTENSION: &str = "shuffled.data";

    pub fn written_beside(input: &Path, seed: ShuffleSeed) -> Result<ShuffledData, TrainerError> {
        let bytes = fs::read(input).map_err(TrainerError::Read)?;
        let (records, remainder) = bytes.as_chunks::<{ Self::RECORD_BYTES }>();
        let mut order: Vec<usize> = (0..records.len()).collect();
        order.shuffle(&mut StdRng::seed_from_u64(u64::from(seed)));
        let path = input.with_extension(Self::EXTENSION);
        if remainder.is_empty() {
            let mut output = BufWriter::new(File::create(&path).map_err(TrainerError::Write)?);
            order
                .into_iter()
                .filter_map(|index| records.get(index))
                .try_for_each(|record| output.write_all(record))
                .and_then(|()| output.flush())
                .map_err(TrainerError::Write)?;
            Ok(ShuffledData { path })
        } else {
            Err(TrainerError::Truncated)
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}
