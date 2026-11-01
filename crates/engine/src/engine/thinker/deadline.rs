use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use board::NodeCount;
use search::Interrupt;

pub(super) struct Deadline<'flag> {
    stop: &'flag AtomicBool,
    hard: Option<Instant>,
    node_limit: Option<NodeCount>,
}

impl<'flag> Deadline<'flag> {
    const CHECK_EVERY: NodeCount = NodeCount::new(1024);

    pub(super) const fn new(
        stop: &'flag AtomicBool,
        hard: Option<Instant>,
        node_limit: Option<NodeCount>,
    ) -> Self {
        Deadline {
            stop,
            hard,
            node_limit,
        }
    }
}

impl Interrupt for Deadline<'_> {
    fn should_stop(&self, nodes: NodeCount) -> bool {
        nodes.count().is_multiple_of(Self::CHECK_EVERY.count())
            && (self.stop.load(Ordering::Relaxed)
                || self.hard.is_some_and(|hard| Instant::now() >= hard)
                || self.node_limit.is_some_and(|limit| nodes >= limit))
    }
}
