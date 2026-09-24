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
pub use table::{BoundKind, TableEntry, TranspositionTable};
pub(crate) use window::{Bound, Lower, Window};

use core::marker::PhantomData;
use core::ops::ControlFlow;

use board::{Board, ChessMove, MoveKind};
use eval::{Evaluator, Score};

use super::depth::Depth;
use super::interrupt::Interrupt;
use full_width::FullWidth;
use killer_table::KillerTable;
use node::Node;
use ordered_moves::OrderedMoves;
use progress::Progress;
use quiescence::Quiescence;
use regime::Regime;
use window::Bounds;

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
            Some(remaining) => self.node::<FullWidth>(node.at_depth(remaining)),
            None if board.in_check() => self.node::<FullWidth>(node),
            None => self.node::<Quiescence>(node),
        }
    }

    pub(crate) fn terminal(board: &Board, distance: RootDistance) -> Score {
        if board.in_check() {
            distance.mated_here()
        } else {
            Score::DRAW
        }
    }

    pub(crate) fn root_moves(&self, board: &Board, principal: Option<ChessMove>) -> OrderedMoves {
        OrderedMoves::from_board(board, self.killers.at_ply(RootDistance::ROOT), principal)
    }

    fn node<R: Regime>(&mut self, node: Node<'_>) -> Score {
        let board = node.board();
        let distance = node.distance();
        let floor = R::floor::<E>(board);
        if node.window().upper().excludes(floor) {
            return floor;
        }
        let moves =
            OrderedMoves::from_board(board, self.killers.at_ply(distance), node.hash_move());
        if moves.is_empty() {
            return Self::terminal(board, distance);
        }
        let searched = moves
            .into_iter()
            .filter(|chess_move| R::considers(*chess_move, board))
            .filter_map(|chess_move| board.make_move(chess_move).map(|child| (chess_move, child)))
            .try_fold(
                Bounds::new(node.window(), floor),
                |bounds, (chess_move, child)| {
                    let window = bounds.child_window();
                    let score = -self.score(&child, node.depth(), distance.deeper(), window);
                    let admitted = bounds.admit(chess_move, score);
                    if admitted.is_break() && !chess_move.captures(board.placement()) {
                        self.killers.remember(distance, chess_move);
                    }
                    admitted
                },
            );
        let conclusion = match searched {
            ControlFlow::Break(bounds) | ControlFlow::Continue(bounds) => bounds.conclude(),
        };
        self.table.store(TableEntry::remember(
            board.hash(),
            node.depth().incremented(),
            distance,
            conclusion,
        ));
        conclusion.score()
    }
}
