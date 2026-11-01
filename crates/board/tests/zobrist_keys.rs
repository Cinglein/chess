use board::{Board, Square, Zobrist};
use std::collections::HashSet;
use strum::IntoEnumIterator;

const RANKS: usize = 8;
const FILES: usize = 8;
const PIECES: [char; 12] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];
const SIDES: [&str; 2] = ["w", "b"];
const RIGHTS: [&str; 4] = ["K", "Q", "k", "q"];
const EN_PASSANT: [&str; 8] = ["a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3"];
const EMPTY: &str = "8/8/8/8/8/8/8/8";

struct Fixture;

impl Fixture {
    fn hash(placement: &str, side: &str, rights: &str, target: &str) -> Zobrist {
        format!("{placement} {side} {rights} {target} 0 1")
            .parse::<Board>()
            .unwrap()
            .hash()
    }

    fn placement(piece: char, square: Square) -> String {
        let index = usize::from(square.to_string().as_bytes()[0] - b'a')
            + FILES * usize::from(square.to_string().as_bytes()[1] - b'1');
        let row = RANKS - 1 - index / FILES;
        let file = index % FILES;
        (0..RANKS)
            .map(|rank| {
                if rank == row {
                    Self::rank_with(piece, file)
                } else {
                    FILES.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("/")
    }

    fn rank_with(piece: char, file: usize) -> String {
        let run = |empty: usize| {
            if empty > 0 {
                empty.to_string()
            } else {
                String::new()
            }
        };
        format!("{}{piece}{}", run(file), run(FILES - 1 - file))
    }

    fn keys() -> Vec<u64> {
        let empty = Self::hash(EMPTY, SIDES[0], "-", "-").bits();
        let pieces = PIECES.into_iter().flat_map(|piece| {
            Square::iter()
                .map(move |square| Self::hash(&Self::placement(piece, square), SIDES[0], "-", "-"))
        });
        let rights = RIGHTS
            .into_iter()
            .map(|right| Self::hash(EMPTY, SIDES[0], right, "-"));
        let targets = EN_PASSANT
            .into_iter()
            .map(|target| Self::hash(EMPTY, SIDES[1], "-", target));
        pieces
            .chain(rights)
            .map(|hash| hash.bits() ^ empty)
            .chain(targets.map(|hash| hash.bits() ^ Self::hash(EMPTY, SIDES[1], "-", "-").bits()))
            .chain([Self::hash(EMPTY, SIDES[1], "-", "-").bits() ^ empty])
            .collect()
    }
}

#[test]
fn every_key_is_distinct_and_nonzero() {
    let all = Fixture::keys();
    let distinct: HashSet<u64> = all.iter().copied().collect();
    assert_eq!(distinct.len(), all.len());
    assert!(!distinct.contains(&0));
}
