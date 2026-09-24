use board::Board;

use crate::score::Score;

pub trait Evaluator {
    #[must_use]
    fn evaluate(board: &Board) -> Score;
}
