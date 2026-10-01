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

    #[test]
    fn a_node_limit_stops_the_search_at_the_first_check_point_past_it() {
        let stop = AtomicBool::new(false);
        let limited = Deadline::new(&stop, None, Some(LIMIT));
        assert_eq!(
            (
                limited.should_stop(CHECK),
                limited.should_stop(LIMIT),
                limited.should_stop(OFF_CHECK)
            ),
            (false, true, false)
        );
    }

    #[test]
    fn a_passed_deadline_or_a_raised_flag_stops_the_search_at_a_check_point() {
        let stop = AtomicBool::new(false);
        let expired = Deadline::new(&stop, Some(Instant::now()), None);
        assert!(expired.should_stop(CHECK) && !Deadline::new(&stop, None, None).should_stop(CHECK));
        stop.store(true, Ordering::Relaxed);
        assert!(Deadline::new(&stop, None, None).should_stop(CHECK));
    }
}
