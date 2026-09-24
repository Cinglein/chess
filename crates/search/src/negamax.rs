use core::marker::PhantomData;
use core::ops::ControlFlow;

use board::Board;
use eval::{Evaluator, Score};

use crate::bounds::Bounds;
use crate::depth::Depth;
use crate::full_width::FullWidth;
use crate::quiescence::Quiescence;
use crate::regime::Regime;
use crate::window::Window;

pub(crate) struct Negamax<E: Evaluator> {
    nodes: u64,
    evaluator: PhantomData<E>,
}

impl<E: Evaluator> Negamax<E> {
    pub(crate) const fn new() -> Negamax<E> {
        Negamax {
            nodes: 0,
            evaluator: PhantomData,
        }
    }

    pub(crate) const fn nodes(&self) -> u64 {
        self.nodes
    }

    pub(crate) fn score(&mut self, board: &Board, depth: Depth, ply: u8, window: Window) -> Score {
        self.nodes += 1;
        match depth.decremented() {
            Some(remaining) => self.node::<FullWidth>(board, remaining, ply, window),
            None if board.in_check() => self.node::<FullWidth>(board, Depth::ZERO, ply, window),
            None => self.node::<Quiescence>(board, Depth::ZERO, ply, window),
        }
    }

    pub(crate) fn terminal(board: &Board, ply: u8) -> Score {
        if board.in_check() {
            Score::mated_in(ply)
        } else {
            Score::DRAW
        }
    }

    fn node<R: Regime>(
        &mut self,
        board: &Board,
        remaining: Depth,
        ply: u8,
        window: Window,
    ) -> Score {
        let floor = R::floor::<E>(board);
        if window.upper().excludes(floor) {
            return floor;
        }
        let moves = board.legal_moves();
        if moves.is_empty() {
            return Self::terminal(board, ply);
        }
        let searched = moves
            .into_iter()
            .filter(|chess_move| R::considers(*chess_move, board))
            .filter_map(|chess_move| board.make_move(chess_move))
            .try_fold(Bounds::new(window, floor), |bounds, child| {
                let reply = ply.saturating_add(1);
                bounds.admit(-self.score(&child, remaining, reply, bounds.child_window()))
            });
        match searched {
            ControlFlow::Break(best) => best,
            ControlFlow::Continue(bounds) => bounds.best(),
        }
    }
}
