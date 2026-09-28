use core::ops::ControlFlow;

use board::{Board, ChessMove, MoveKind};
use eval::{Evaluator, Score};

use super::super::depth::Depth;
use super::super::interrupt::Interrupt;
use super::Negamax;
use super::ordered_moves::OrderedMoves;
use super::regime::Regime;
use super::table::{RootDistance, TableEntry};
use super::window::{Bounds, Window};

#[derive(Clone, Copy)]
pub(crate) struct Node<'position> {
    board: &'position Board,
    depth: Depth,
    distance: RootDistance,
    window: Window,
    hash_move: Option<ChessMove>,
}

impl<'position> Node<'position> {
    pub(crate) const fn new(
        board: &'position Board,
        depth: Depth,
        distance: RootDistance,
        window: Window,
    ) -> Self {
        Node {
            board,
            depth,
            distance,
            window,
            hash_move: None,
        }
    }

    pub(crate) const fn remembering(self, hash_move: Option<ChessMove>) -> Self {
        Node { hash_move, ..self }
    }

    pub(crate) const fn at_depth(self, depth: Depth) -> Self {
        Node { depth, ..self }
    }

    pub(crate) fn search<R: Regime, E: Evaluator, I: Interrupt>(
        self,
        regime: &R,
        negamax: &mut Negamax<'_, '_, '_, E, I>,
    ) -> Score {
        let floor = regime.floor::<E>(self.board);
        if self.window.upper().excludes(floor) {
            return floor;
        }
        let moves = OrderedMoves::from_board(
            self.board,
            negamax.killers.at_ply(self.distance),
            self.hash_move,
        );
        if moves.is_empty() {
            return Negamax::<E, I>::terminal(self.board, self.distance);
        }
        let searched = moves
            .into_iter()
            .filter(|chess_move| regime.considers(*chess_move, self.board))
            .filter_map(|chess_move| {
                self.board
                    .make_move(chess_move)
                    .map(|child| (chess_move, child))
            })
            .try_fold(
                Bounds::new(self.window, floor),
                |bounds, (chess_move, child)| {
                    let window = bounds.child_window();
                    let score = -negamax.score(&child, self.depth, self.distance.deeper(), window);
                    let admitted = bounds.admit(chess_move, score);
                    if admitted.is_break() && !chess_move.captures(self.board.placement()) {
                        negamax.killers.remember(self.distance, chess_move);
                    }
                    admitted
                },
            );
        let conclusion = match searched {
            ControlFlow::Break(bounds) | ControlFlow::Continue(bounds) => bounds.conclude(),
        };
        negamax.table.store(TableEntry::remember(
            self.board.hash(),
            self.depth.incremented(),
            self.distance,
            conclusion,
        ));
        conclusion.score()
    }
}
