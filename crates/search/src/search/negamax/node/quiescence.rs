use board::{Board, ChessMove, MoveKind};
use eval::{Evaluator, Score};

use super::regime::Regime;

pub(crate) struct Quiescence;

impl Regime for Quiescence {
    fn floor<E: Evaluator>(&self, board: &Board) -> Score {
        E::evaluate(board)
    }

    fn considers(&self, chess_move: ChessMove, board: &Board) -> bool {
        chess_move.captures(board.placement())
    }
}
