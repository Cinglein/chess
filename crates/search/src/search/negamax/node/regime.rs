use board::{Board, ChessMove};
use eval::{Evaluator, Score};

pub(crate) trait Regime {
    fn floor<E: Evaluator>(&self, board: &Board) -> Score;

    fn considers(&self, chess_move: ChessMove, board: &Board) -> bool;
}
