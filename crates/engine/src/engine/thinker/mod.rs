mod deadline;
mod time_budget;

use std::ops::ControlFlow;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use board::Board;
use deadline::Deadline;
use eval::PieceSquareTables;
use search::{Depth, Search, TableEntry, TranspositionTable};
use time_budget::TimeBudget;
use uci::{GoLimits, Response, SearchInfo};

use super::sink::Sink;

pub(super) struct Thinker<'position, 'flag> {
    board: &'position Board,
    stop: &'flag AtomicBool,
}

impl<'position, 'flag> Thinker<'position, 'flag> {
    const MAX_DEPTH: Depth = Depth::new(63);

    pub(super) const fn new(board: &'position Board, stop: &'flag AtomicBool) -> Self {
        Thinker { board, stop }
    }

    pub(super) fn think<S: Sink>(
        &self,
        entries: &mut [TableEntry],
        limits: &GoLimits,
        sink: &mut S,
    ) {
        let start = Instant::now();
        let allowance = limits.move_time().or_else(|| {
            limits
                .clock()
                .map(|clock| TimeBudget::allot(&clock, self.board.side_to_move()))
        });
        let deadline = Deadline::new(
            self.stop,
            allowance.map(|budget| start + budget),
            limits.nodes(),
        );
        let mut table = TranspositionTable::new(entries);
        let plies = limits.depth().unwrap_or(Self::MAX_DEPTH).plies();
        let searched = (0..plies).try_fold(
            Search::<PieceSquareTables>::from(*self.board),
            |search, _| {
                let before = search.depth();
                let deepened = search.deepen(&mut table, &deadline);
                if deepened.depth() == before {
                    return ControlFlow::Break(deepened);
                }
                sink.emit(Response::Info(SearchInfo::from_search(
                    &deepened,
                    start.elapsed(),
                )));
                if allowance.is_some_and(|budget| start.elapsed() * 2 >= budget) {
                    ControlFlow::Break(deepened)
                } else {
                    ControlFlow::Continue(deepened)
                }
            },
        );
        let search = match searched {
            ControlFlow::Break(search) | ControlFlow::Continue(search) => search,
        };
        sink.emit(Response::BestMove(search.best_move()));
    }
}
