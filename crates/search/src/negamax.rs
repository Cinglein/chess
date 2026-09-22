use core::marker::PhantomData;
use core::ops::ControlFlow;

use board::Board;
use eval::{Evaluator, Score};

use crate::depth::Depth;

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

    pub(crate) fn score(
        &mut self,
        board: &Board,
        depth: Depth,
        ply: u8,
        alpha: Score,
        beta: Score,
    ) -> Score {
        self.nodes += 1;
        let Some(remaining) = depth.decremented() else {
            return E::evaluate(board);
        };
        let moves = board.legal_moves();
        if moves.is_empty() {
            return Self::terminal(board, ply);
        }
        let searched = moves
            .into_iter()
            .filter_map(|chess_move| board.make_move(chess_move))
            .try_fold((alpha, -Score::INFINITY), |(alpha, best), child| {
                let score = -self.score(&child, remaining, ply.saturating_add(1), -beta, -alpha);
                let best = best.max(score);
                if score >= beta {
                    ControlFlow::Break(best)
                } else {
                    ControlFlow::Continue((alpha.max(score), best))
                }
            });
        match searched {
            ControlFlow::Break(best) | ControlFlow::Continue((_, best)) => best,
        }
    }

    pub(crate) fn terminal(board: &Board, ply: u8) -> Score {
        if board.in_check() {
            Score::mated_in(ply)
        } else {
            Score::DRAW
        }
    }
}
