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

#[cfg(test)]
mod tests {
    use super::super::super::Negamax;
    use super::super::super::table::{RootDistance, TranspositionTable};
    use super::super::super::window::Window;
    use super::{Board, Evaluator, MoveKind, Score};
    use crate::search::depth::Depth;
    use crate::search::{Opening, Uninterrupted};
    use core::cell::Cell;
    use eval::PieceSquareTables;
    use proptest::prelude::*;
    const CASES: u32 = 32;

    struct Oracle {
        visited: Cell<u32>,
    }

    impl Oracle {
        const AFFORDABLE_NODES: u32 = 1 << 12;

        fn new() -> Oracle {
            Oracle {
                visited: Cell::new(0),
            }
        }

        fn score(&self, board: &Board, distance: RootDistance) -> Option<Score> {
            self.visited.set(self.visited.get() + 1);
            let moves = board.legal_moves();
            let floor = if board.in_check() {
                Score::INFINITY.negated()
            } else {
                PieceSquareTables::evaluate(board)
            };
            if self.visited.get() > Self::AFFORDABLE_NODES {
                None
            } else if moves.is_empty() && board.in_check() {
                Some(-Score::mate_in(distance.plies()))
            } else if moves.is_empty() {
                Some(Score::DRAW)
            } else {
                moves
                    .into_iter()
                    .filter(|chess_move| board.in_check() || chess_move.captures(board.placement()))
                    .filter_map(|chess_move| board.make_move(chess_move))
                    .try_fold(floor, |best, after| {
                        Some(best.max(-self.score(&after, distance.deeper())?))
                    })
            }
        }
    }

    #[test]
    fn a_leaf_scores_as_the_plain_stand_pat_capture_search_with_every_check_answered_in_full() {
        proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards())| {
            let expected = Oracle::new().score(&board, RootDistance::ROOT);
            prop_assume!(expected.is_some());
            let mut table = TranspositionTable::new(&mut []);
            let mut negamax = Negamax::<PieceSquareTables, Uninterrupted>::new(&mut table, &Uninterrupted);
            let scored = negamax.score(&board, Depth::ZERO, RootDistance::ROOT, Window::FULL);
            prop_assert_eq!(Some(scored), expected, "{}", board);
        });
    }
}
