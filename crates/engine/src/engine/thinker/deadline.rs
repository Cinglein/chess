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

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Instant;

    use board::NodeCount;
    use search::Interrupt;

    use super::Deadline;

    const CHECK: NodeCount = NodeCount::new(1 << 10);
    const LIMIT: NodeCount = NodeCount::new(1 << 11);
    const OFF_CHECK: NodeCount = NodeCount::new((1 << 11) + 1);
    const AT_CHECK_LIMIT_AND_PAST: (bool, bool, bool) = (false, true, false);

    #[test]
    fn a_search_stops_at_a_check_point_once_its_node_limit_deadline_or_stop_flag_is_reached() {
        let stop = AtomicBool::default();
        let limited = Deadline::new(&stop, None, Some(LIMIT));
        assert_eq!(
            (
                limited.should_stop(CHECK),
                limited.should_stop(LIMIT),
                limited.should_stop(OFF_CHECK)
            ),
            AT_CHECK_LIMIT_AND_PAST
        );
        let expired = Deadline::new(&stop, Some(Instant::now()), None);
        assert!(expired.should_stop(CHECK) && !Deadline::new(&stop, None, None).should_stop(CHECK));
        stop.store(true, Ordering::Relaxed);
        assert!(Deadline::new(&stop, None, None).should_stop(CHECK));
    }
}
