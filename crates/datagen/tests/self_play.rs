use arena::{RandomOpenings, Seed, WorkerCount};
use board::{FullmoveNumber, NodeCount, PlyCount};
use datagen::{PositionCount, SelfPlay};
use game::{Finished, Rules};
use std::num::NonZeroU16;
const SEED: u64 = 3;
const PLIES: PlyCount = PlyCount::new(6);
const SHORT: Rules = Rules::at_fixed_nodes(NodeCount::new(64))
    .lasting_at_most(FullmoveNumber::new(NonZeroU16::new(6).unwrap()));
const ONE: PositionCount = PositionCount::new(1);

struct Fixture;

impl Fixture {
    fn games_until(target: PositionCount) -> Vec<Finished> {
        SelfPlay::new(
            RandomOpenings::seeded(Seed::from(SEED), PLIES),
            SHORT,
            target,
        )
        .play(WorkerCount::from(1))
        .into_iter()
        .collect()
    }
}

#[test]
fn self_play_stops_at_the_game_that_reaches_the_target_and_replays_the_same_games_from_a_seed() {
    let first = Fixture::games_until(ONE);
    let labels = PositionCount::new(first.iter().map(|game| game.labels().count()).sum());
    let exact = Fixture::games_until(labels);
    let more = Fixture::games_until(labels + ONE);
    assert_eq!((first.len(), exact.len(), more.len()), (1, 1, 2));
    assert_eq!(
        first.first().map(ToString::to_string),
        exact.first().map(ToString::to_string)
    );
}
