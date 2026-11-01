mod full_width;
mod quiescence;
mod regime;

use core::ops::ControlFlow;

use board::{Board, ChessMove};
use eval::{Evaluator, Score};

use super::super::depth::Depth;
use super::super::interrupt::Interrupt;
use super::Negamax;
use super::ordered_moves::OrderedMoves;
use super::table::{Conclusion, RootDistance, TableEntry};
use super::window::{Bounds, Window};
use full_width::FullWidth;
use quiescence::Quiescence;
use regime::Regime;

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

    pub(crate) fn search<R: Regime, E: Evaluator, I: Interrupt>(
        self,
        regime: &R,
        negamax: &mut Negamax<'_, '_, '_, E, I>,
    ) -> Score {
        let floor = regime.floor::<E>(self.board);
        if self.window.upper().excludes(floor) {
            floor
        } else {
            self.explored(regime, negamax, floor)
        }
    }

    pub(crate) fn explore<E: Evaluator, I: Interrupt>(
        self,
        negamax: &mut Negamax<'_, '_, '_, E, I>,
    ) -> Score {
        match self.depth.decremented() {
            Some(_) => self.search(&FullWidth, negamax),
            None if self.board.in_check() => self.search(&FullWidth, negamax),
            None => self.search(&Quiescence, negamax),
        }
    }

    fn explored<R: Regime, E: Evaluator, I: Interrupt>(
        self,
        regime: &R,
        negamax: &mut Negamax<'_, '_, '_, E, I>,
        floor: Score,
    ) -> Score {
        let moves = OrderedMoves::from_board(
            self.board,
            negamax.killers.at_ply(self.distance),
            self.hash_move,
        );
        if moves.is_empty() {
            Negamax::<E, I>::terminal(self.board, self.distance)
        } else {
            let conclusion = self.best_of(regime, negamax, moves, floor);
            negamax.table.store(TableEntry::remember(
                self.board.hash(),
                self.depth,
                self.distance,
                conclusion,
            ));
            conclusion.score()
        }
    }

    fn best_of<R: Regime, E: Evaluator, I: Interrupt>(
        self,
        regime: &R,
        negamax: &mut Negamax<'_, '_, '_, E, I>,
        moves: OrderedMoves,
        floor: Score,
    ) -> Conclusion {
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
                    let score = -negamax.score(
                        &child,
                        self.depth.decremented().unwrap_or(Depth::ZERO),
                        self.distance.deeper(),
                        window,
                    );
                    bounds.admit(chess_move, score).map_break(|cut| {
                        negamax
                            .killers
                            .remember_quiet(self.distance, chess_move, self.board);
                        cut
                    })
                },
            );
        match searched {
            ControlFlow::Break(bounds) | ControlFlow::Continue(bounds) => bounds.conclude(),
        }
    }
}
