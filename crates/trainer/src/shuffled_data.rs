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
        order.shuffle(&mut StdRng::seed_from_u64(seed.bits()));
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::{ShuffleSeed, ShuffledData};

    const RECORDS: usize = 16;
    const SEED: ShuffleSeed = ShuffleSeed::new(5);
    const OTHER_SEED: ShuffleSeed = ShuffleSeed::new(6);

    struct Fixture;

    impl Fixture {
        fn data_file(name: &str, bytes: usize) -> PathBuf {
            let path = std::env::temp_dir().join(name);
            let content: Vec<u8> = (0..bytes)
                .map(|offset| u8::try_from(offset / ShuffledData::RECORD_BYTES).unwrap())
                .collect();
            fs::write(&path, content).unwrap();
            path
        }

        fn first_bytes(path: &std::path::Path) -> Vec<u8> {
            fs::read(path)
                .unwrap()
                .as_chunks::<{ ShuffledData::RECORD_BYTES }>()
                .0
                .iter()
                .map(|record| record[0])
                .collect()
        }
    }

    #[test]
    fn shuffling_permutes_whole_records_reproducibly_from_a_seed() {
        let input = Fixture::data_file("whole.data", RECORDS * ShuffledData::RECORD_BYTES);
        let shuffled = ShuffledData::written_beside(&input, SEED).unwrap();
        let again = ShuffledData::written_beside(&input, SEED).unwrap();
        let order = Fixture::first_bytes(shuffled.path());
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(
            (sorted, &order),
            (
                Fixture::first_bytes(&input),
                &Fixture::first_bytes(again.path())
            )
        );
        let other = ShuffledData::written_beside(&input, OTHER_SEED).unwrap();
        assert!(
            order != Fixture::first_bytes(&input) && order != Fixture::first_bytes(other.path())
        );
    }

    #[test]
    fn a_file_that_ends_mid_record_is_refused() {
        let input = Fixture::data_file("torn.data", RECORDS * ShuffledData::RECORD_BYTES + 1);
        assert!(ShuffledData::written_beside(&input, SEED).is_err());
    }
}
