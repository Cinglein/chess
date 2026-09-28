mod full_width;
mod killer_table;
mod node;
mod ordered_moves;
mod progress;
mod quiescence;
mod regime;
mod table;
mod window;

pub(crate) use table::RootDistance;
pub use table::{TableEntry, TranspositionTable};
pub(crate) use window::{Bound, Lower, Window};

use core::marker::PhantomData;

use board::{Board, ChessMove};
use eval::{Evaluator, Score};

use super::depth::Depth;
use super::interrupt::Interrupt;
use full_width::FullWidth;
use killer_table::KillerTable;
use node::Node;
use ordered_moves::OrderedMoves;
use progress::Progress;
use quiescence::Quiescence;

pub(crate) struct Negamax<'store, 'table, 'stop, E: Evaluator, I: Interrupt> {
    nodes: u64,
    killers: KillerTable,
    table: &'table mut TranspositionTable<'store>,
    interrupt: &'stop I,
    progress: Progress,
    evaluator: PhantomData<E>,
}

impl<'store, 'table, 'stop, E: Evaluator, I: Interrupt> Negamax<'store, 'table, 'stop, E, I> {
    pub(crate) const fn new(
        table: &'table mut TranspositionTable<'store>,
        interrupt: &'stop I,
    ) -> Self {
        Negamax {
            nodes: 0,
            killers: KillerTable::new(),
            table,
            interrupt,
            progress: Progress::Running,
            evaluator: PhantomData,
        }
    }

    pub(crate) const fn nodes(&self) -> u64 {
        self.nodes
    }

    pub(crate) fn was_aborted(&self) -> bool {
        self.progress == Progress::Aborted
    }

    pub(crate) fn score(
        &mut self,
        board: &Board,
        depth: Depth,
        distance: RootDistance,
        window: Window,
    ) -> Score {
        self.nodes += 1;
        if self.was_aborted() || self.interrupt.should_stop(self.nodes) {
            self.progress = Progress::Aborted;
            return Score::DRAW;
        }
        let remembered = self.table.probe(board.hash());
        if let Some(score) = remembered.and_then(|entry| entry.settles(depth, distance, window)) {
            return score;
        }
        let node = Node::new(board, depth, distance, window)
            .remembering(remembered.and_then(|entry| entry.best_move()));
        match depth.decremented() {
            Some(remaining) => node.at_depth(remaining).search(&FullWidth, self),
            None if board.in_check() => node.search(&FullWidth, self),
            None => node.search(&Quiescence, self),
        }
    }

    pub(crate) fn terminal(board: &Board, distance: RootDistance) -> Score {
        if board.in_check() {
            Score::mated_in(distance.plies())
        } else {
            Score::DRAW
        }
    }

    pub(crate) fn root_moves(&self, board: &Board, principal: Option<ChessMove>) -> OrderedMoves {
        OrderedMoves::from_board(board, self.killers.at_ply(RootDistance::ROOT), principal)
    }
}
