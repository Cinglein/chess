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
