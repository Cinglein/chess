use board::{Board, ChessMove};
use eval::{Evaluator, Score};

use crate::regime::Regime;

pub(crate) struct FullWidth;

impl Regime for FullWidth {
    fn floor<E: Evaluator>(_: &Board) -> Score {
        Score::INFINITY.negated()
    }

    fn considers(_: ChessMove, _: &Board) -> bool {
        true
    }
}
