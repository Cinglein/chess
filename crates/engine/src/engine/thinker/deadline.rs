use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use search::Interrupt;

pub(super) struct Deadline<'flag> {
    stop: &'flag AtomicBool,
    hard: Option<Instant>,
    node_limit: Option<u64>,
}

impl<'flag> Deadline<'flag> {
    const CHECK_EVERY: u64 = 1024;

    pub(super) const fn new(
        stop: &'flag AtomicBool,
        hard: Option<Instant>,
        node_limit: Option<u64>,
    ) -> Self {
        Deadline {
            stop,
            hard,
            node_limit,
        }
    }
}

impl Interrupt for Deadline<'_> {
    fn should_stop(&self, nodes: u64) -> bool {
        nodes.is_multiple_of(Self::CHECK_EVERY)
            && (self.stop.load(Ordering::Relaxed)
                || self.hard.is_some_and(|hard| Instant::now() >= hard)
                || self.node_limit.is_some_and(|limit| nodes >= limit))
    }
}
