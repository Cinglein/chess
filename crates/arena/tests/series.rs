use arena::{Entrant, Series, WorkerCount};
use board::{Board, NodeCount};
use game::{ArenaError, Opponent, Rules};
use opponent_ports::InProcessEngine;
use std::sync::atomic::{AtomicUsize, Ordering};
const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
const QUICK: Rules = Rules::at_fixed_nodes(NodeCount::new(64));
const TALLIED: &str = "+2 =0 -2 2/4 Elo +0 (+0 to +0) inconclusive";
const PAIRS: usize = 2;
const GAMES: usize = 2 * PAIRS;

struct InProcess;

impl Entrant for InProcess {
    fn opponent(&self) -> Result<Box<dyn Opponent>, ArenaError> {
        Ok(Box::new(InProcessEngine::default()))
    }
}

#[test]
fn a_series_plays_each_opening_from_both_sides_reports_every_game_and_tallies_for_the_challenger() {
    let openings = vec![MATE_IN_ONE.parse::<Board>().unwrap(); PAIRS];
    let reported = AtomicUsize::new(0);
    let tally = Series::new(&InProcess, &InProcess, QUICK)
        .play(openings, WorkerCount::one_per_core(), &|_| {
            reported.fetch_add(1, Ordering::Relaxed);
        })
        .unwrap();
    assert_eq!(
        (tally.to_string(), reported.into_inner()),
        (TALLIED.to_owned(), GAMES)
    );
}
