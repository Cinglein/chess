use std::iter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, SendError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::position_count::PositionCount;
use arena::{Finished, Game, InProcessEngine, RandomOpenings, Rules, WorkerCount};

pub struct SelfPlay {
    openings: Mutex<RandomOpenings>,
    produced: AtomicUsize,
    target: PositionCount,
    rules: Rules,
}

impl SelfPlay {
    #[must_use]
    pub fn new(openings: RandomOpenings, rules: Rules, target: PositionCount) -> SelfPlay {
        SelfPlay {
            openings: Mutex::new(openings),
            produced: AtomicUsize::new(PositionCount::default().count()),
            target,
            rules,
        }
    }

    #[must_use]
    pub fn play(self, concurrency: WorkerCount) -> Receiver<Finished> {
        let (sender, receiver) = mpsc::channel();
        let shared = Arc::new(self);
        let workers: Vec<_> = (0..concurrency.at_least_one())
            .map(|_| {
                let shared = Arc::clone(&shared);
                let sender = sender.clone();
                thread::spawn(move || shared.work(&sender))
            })
            .collect();
        drop(workers);
        receiver
    }

    fn work(&self, sender: &Sender<Finished>) -> Result<(), SendError<Finished>> {
        let mut white = InProcessEngine::default();
        let mut black = InProcessEngine::default();
        iter::from_fn(|| {
            (self.produced.load(Ordering::Relaxed) < self.target.count())
                .then(|| self.openings.lock().ok()?.next())
                .flatten()
        })
        .map(|opening| {
            Game::from(opening)
                .ruled_by(self.rules)
                .play(&mut white, &mut black)
        })
        .try_for_each(|finished| {
            self.produced
                .fetch_add(finished.labels().count(), Ordering::Relaxed);
            sender.send(finished)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use arena::{Finished, RandomOpenings, Rules, Seed, Thinking, WorkerCount};
    use board::{FullmoveNumber, NodeCount, PlyCount};

    use super::{PositionCount, SelfPlay};

    const SEED: Seed = Seed::new(3);
    const PLIES: PlyCount = PlyCount::new(6);
    const SHORT: Rules = Rules::DEFAULT
        .thinking_by(Thinking::FixedNodes(NodeCount::new(64)))
        .lasting_at_most(FullmoveNumber::new(NonZeroU16::new(6).unwrap()));
    const ONE: PositionCount = PositionCount::new(1);

    impl SelfPlay {
        fn games_until(target: PositionCount) -> Vec<Finished> {
            SelfPlay::new(RandomOpenings::seeded(SEED, PLIES), SHORT, target)
                .play(WorkerCount::new(1))
                .into_iter()
                .collect()
        }
    }

    #[test]
    fn self_play_stops_at_the_game_that_reaches_the_target_and_replays_the_same_games_from_a_seed()
    {
        let first = SelfPlay::games_until(ONE);
        let labels = PositionCount::new(first.iter().map(|game| game.labels().count()).sum());
        let exact = SelfPlay::games_until(labels);
        let more = SelfPlay::games_until(labels + ONE);
        assert_eq!((first.len(), exact.len(), more.len()), (1, 1, 2));
        assert_eq!(
            first.first().map(ToString::to_string),
            exact.first().map(ToString::to_string)
        );
    }
}
