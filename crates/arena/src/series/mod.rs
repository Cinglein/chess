mod tally;

use std::collections::VecDeque;
use std::iter;
use std::sync::Mutex;
use std::thread::{self, Scope, ScopedJoinHandle};

use board::Board;
use tally::Tally;

use crate::arena_error::ArenaError;
use crate::entrant::Entrant;
use crate::game::{Finished, Game};
use crate::rules::Rules;
use crate::worker_count::WorkerCount;

pub struct Series<'entrants> {
    challenger: &'entrants dyn Entrant,
    reference: &'entrants dyn Entrant,
    rules: Rules,
}

impl<'entrants> Series<'entrants> {
    #[must_use]
    pub const fn new(
        challenger: &'entrants dyn Entrant,
        reference: &'entrants dyn Entrant,
        rules: Rules,
    ) -> Series<'entrants> {
        Series {
            challenger,
            reference,
            rules,
        }
    }

    pub fn play(
        &self,
        openings: Vec<Board>,
        concurrency: WorkerCount,
        report: &(dyn Fn(&Finished) + Sync),
    ) -> Result<Tally, ArenaError> {
        let queue: Mutex<VecDeque<Board>> = Mutex::new(openings.into_iter().collect());
        thread::scope(|scope| {
            let workers: Vec<ScopedJoinHandle<'_, Result<Tally, ArenaError>>> = (0..concurrency
                .at_least_one())
                .map(|_| self.spawn_worker(scope, &queue, report))
                .collect();
            workers
                .into_iter()
                .try_fold(Tally::default(), |tally, worker| {
                    Ok(tally.merged(Self::joined(worker)?))
                })
        })
    }

    fn spawn_worker<'scope, 'env>(
        &'env self,
        scope: &'scope Scope<'scope, 'env>,
        queue: &'env Mutex<VecDeque<Board>>,
        report: &'env (dyn Fn(&Finished) + Sync),
    ) -> ScopedJoinHandle<'scope, Result<Tally, ArenaError>> {
        scope.spawn(move || self.work(queue, report))
    }

    fn joined(
        worker: ScopedJoinHandle<'_, Result<Tally, ArenaError>>,
    ) -> Result<Tally, ArenaError> {
        worker.join().map_err(|_| ArenaError::Worker)?
    }

    fn work(
        &self,
        queue: &Mutex<VecDeque<Board>>,
        report: &(dyn Fn(&Finished) + Sync),
    ) -> Result<Tally, ArenaError> {
        let mut challenger = self.challenger.opponent()?;
        let mut reference = self.reference.opponent()?;
        Ok(iter::from_fn(|| queue.lock().ok()?.pop_front()).fold(
            Tally::default(),
            |tally, opening| {
                let as_white = Game::from(opening)
                    .ruled_by(self.rules)
                    .play(challenger.as_mut(), reference.as_mut());
                report(&as_white);
                let as_black = Game::from(opening)
                    .ruled_by(self.rules)
                    .play(reference.as_mut(), challenger.as_mut());
                report(&as_black);
                tally.recorded_pair(as_white.outcome().verdict(), as_black.outcome().verdict())
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use board::{Board, NodeCount};

    use super::{ArenaError, Entrant, Rules, Series, WorkerCount};
    use crate::in_process_engine::InProcessEngine;
    use crate::opponent::Opponent;
    use crate::rules::Thinking;

    const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
    const TALLIED: &str = "+2 =0 -2 2/4 Elo +0 (+0 to +0) inconclusive";
    const PAIRS: usize = 2;
    const GAMES: usize = 2 * PAIRS;
    const QUICK: Rules = Rules::DEFAULT.thinking_by(Thinking::FixedNodes(NodeCount::new(64)));

    struct InProcess;

    impl Entrant for InProcess {
        fn opponent(&self) -> Result<Box<dyn Opponent>, ArenaError> {
            Ok(Box::new(InProcessEngine::default()))
        }
    }

    #[test]
    fn a_series_plays_each_opening_from_both_sides_reports_every_game_and_tallies_for_the_challenger()
     {
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
}
