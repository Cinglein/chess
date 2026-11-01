use bulletformat::ChessBoard;
use std::fs;
use std::path::PathBuf;
use trainer::{ShuffleSeed, ShuffledData};
const RECORDS: usize = 16;
const SEED: &str = "5";
const OTHER_SEED: &str = "6";
const RECORD_BYTES: usize = size_of::<ChessBoard>();

struct Fixture;

impl Fixture {
    fn data_file(name: &str, bytes: usize) -> PathBuf {
        let path = std::env::temp_dir().join(name);
        let content: Vec<u8> = (0..bytes)
            .map(|offset| u8::try_from(offset / RECORD_BYTES).unwrap())
            .collect();
        fs::write(&path, content).unwrap();
        path
    }

    fn first_bytes(path: &std::path::Path) -> Vec<u8> {
        let bytes = fs::read(path).unwrap();
        bytes.chunks(RECORD_BYTES).map(|record| record[0]).collect()
    }
}

#[test]
fn shuffling_permutes_whole_records_reproducibly_from_a_seed() {
    let input = Fixture::data_file("whole.data", RECORDS * RECORD_BYTES);
    let [order, again, other] = [SEED, SEED, OTHER_SEED].map(|seed| {
        let seed: ShuffleSeed = seed.parse().unwrap();
        Fixture::first_bytes(ShuffledData::written_beside(&input, seed).unwrap().path())
    });
    let original = Fixture::first_bytes(&input);
    let mut sorted = order.clone();
    sorted.sort_unstable();
    assert_eq!((&sorted, &again), (&original, &order));
    assert!(order != original && order != other);
}

#[test]
fn a_file_that_ends_mid_record_is_refused() {
    let input = Fixture::data_file("torn.data", RECORDS * RECORD_BYTES + 1);
    assert!(ShuffledData::written_beside(&input, ShuffleSeed::default()).is_err());
}
