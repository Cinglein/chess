use board::{Board, ChessMove};
use eval::{Evaluator, Score};

pub(crate) trait Regime {
    fn floor<E: Evaluator>(board: &Board) -> Score;

    fn considers(chess_move: ChessMove, board: &Board) -> bool;
}
