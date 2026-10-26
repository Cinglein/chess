mod depth;
mod interrupt;
mod negamax;
#[cfg(test)]
mod opening;
mod principal;
#[cfg(test)]
mod uninterrupted;

pub use depth::Depth;
pub use interrupt::Interrupt;
pub use negamax::{TableEntry, TranspositionTable};
#[cfg(test)]
use opening::Opening;
#[cfg(test)]
use uninterrupted::Uninterrupted;

use core::marker::PhantomData;

use board::{Board, ChessMove, NodeCount, State};
use eval::{Evaluator, Score};

use negamax::{Negamax, RootDistance};
use principal::Principal;

pub struct Search<E: Evaluator> {
    board: Board,
    depth: Depth,
    best_move: Option<ChessMove>,
    score: Score,
    nodes: NodeCount,
    evaluator: PhantomData<E>,
}

impl<E: Evaluator> State for Search<E> {}

impl<E: Evaluator> Search<E> {
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }

    #[must_use]
    pub const fn depth(&self) -> Depth {
        self.depth
    }

    #[must_use]
    pub const fn best_move(&self) -> Option<ChessMove> {
        self.best_move
    }

    #[must_use]
    pub const fn score(&self) -> Score {
        self.score
    }

    #[must_use]
    pub const fn nodes(&self) -> NodeCount {
        self.nodes
    }

    #[must_use]
    pub fn deepen<I: Interrupt>(
        self,
        table: &mut TranspositionTable<'_>,
        interrupt: &I,
    ) -> Search<E> {
        let depth = self.depth.incremented();
        let remaining = depth.decremented().unwrap_or_default();
        let mut negamax = Negamax::<E, I>::new(table, interrupt);
        let principal = negamax
            .root_moves(&self.board, self.best_move)
            .into_iter()
            .filter_map(|chess_move| {
                self.board
                    .make_move(chess_move)
                    .map(|child| (chess_move, child))
            })
            .fold(Principal::NONE, |principal, (chess_move, child)| {
                let distance = RootDistance::ROOT.deeper();
                let score = -negamax.score(&child, remaining, distance, principal.window());
                principal.improved(chess_move, score)
            });
        if negamax.was_aborted() {
            self
        } else {
            Search {
                depth,
                best_move: principal.chess_move(),
                score: principal.chess_move().map_or_else(
                    || Negamax::<E, I>::terminal(&self.board, RootDistance::ROOT),
                    |_| principal.score(),
                ),
                nodes: self.nodes + negamax.nodes(),
                ..self
            }
        }
    }
}

impl<E: Evaluator> From<Board> for Search<E> {
    fn from(board: Board) -> Search<E> {
        Search {
            board,
            depth: Depth::ZERO,
            best_move: None,
            score: E::evaluate(&board),
            nodes: NodeCount::ZERO,
            evaluator: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::negamax::Window;
    use super::{
        Negamax, Opening, RootDistance, Search, TableEntry, TranspositionTable, Uninterrupted,
    };
    use crate::search::depth::Depth;
    use board::Board;
    use eval::{PieceSquareTables, Score};
    use proptest::prelude::*;
    const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
    const MATE_IN_TWO: &str = "7k/8/8/8/8/8/R7/1R4K1 w - - 0 1";
    const STALEMATE: &str = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
    const MATES: [(&str, u8, u8); 2] = [(MATE_IN_ONE, 2, 1), (MATE_IN_TWO, 3, 3)];
    const MATING_MOVE: &str = "a1a8";
    const TABLE_ENTRIES: usize = 1 << 16;
    const CASES: u32 = 12;

    struct Fixture;

    impl Fixture {
        fn searched(
            board: Board,
            depth: Depth,
            table: &mut TranspositionTable<'_>,
        ) -> Search<PieceSquareTables> {
            (0..depth.plies()).fold(Search::from(board), |search, _| {
                search.deepen(table, &Uninterrupted)
            })
        }

        fn from_fen(fen: &str, depth: u8) -> Search<PieceSquareTables> {
            let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
            let mut table = TranspositionTable::new(&mut store);
            Self::searched(fen.parse().unwrap(), Depth::new(depth), &mut table)
        }

        fn without_table(board: Board, depth: Depth) -> Search<PieceSquareTables> {
            Self::searched(board, depth, &mut TranspositionTable::new(&mut []))
        }

        fn deepened_once_more(
            board: Board,
            table: &mut TranspositionTable<'_>,
        ) -> Search<PieceSquareTables> {
            Self::without_table(board, Depth::new(1)).deepen(table, &Uninterrupted)
        }
    }

    struct Oracle;

    impl Oracle {
        fn score(board: &Board, depth: Depth, distance: RootDistance) -> Score {
            let moves = board.legal_moves();
            match depth.decremented() {
                None => Negamax::<PieceSquareTables, Uninterrupted>::new(
                    &mut TranspositionTable::new(&mut []),
                    &Uninterrupted,
                )
                .score(board, Depth::ZERO, distance, Window::FULL),
                Some(_) if moves.is_empty() && board.in_check() => {
                    -Score::mate_in(distance.plies())
                }
                Some(_) if moves.is_empty() => Score::DRAW,
                Some(remaining) => moves
                    .into_iter()
                    .filter_map(|chess_move| board.make_move(chess_move))
                    .map(|after| -Self::score(&after, remaining, distance.deeper()))
                    .max()
                    .unwrap(),
            }
        }

        fn reply_score(board: &Board, depth: Depth) -> Score {
            -Self::score(board, depth, RootDistance::ROOT.deeper())
        }
    }

    #[test]
    fn mates_are_found_at_the_depth_that_reaches_them_and_stalemate_has_no_move_and_a_drawn_score()
    {
        for (fen, depth, plies) in MATES {
            let search = Fixture::from_fen(fen, depth);
            assert_eq!(search.score(), Score::mate_in(plies), "{fen}");
        }
        let best = Fixture::from_fen(MATE_IN_ONE, 2).best_move();
        assert_eq!(
            best.map(|chess_move| chess_move.to_string()).as_deref(),
            Some(MATING_MOVE)
        );
        let search = Fixture::from_fen(STALEMATE, 1);
        assert_eq!((search.best_move(), search.score()), (None, Score::DRAW));
    }

    #[test]
    fn alpha_beta_scores_as_the_full_width_tree_over_full_window_leaves_and_its_best_move_earns_that_score()
     {
        proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards(), depth in (1..=2u8).prop_map(Depth::new))| {
            let search = Fixture::without_table(board, depth);
            prop_assert_eq!(search.score(), Oracle::score(&board, depth, RootDistance::ROOT), "{} at {}", board, depth);
            let replied = search.best_move().and_then(|chess_move| board.make_move(chess_move));
            prop_assert_eq!(
                replied.map_or(search.score(), |after| Oracle::reply_score(&after, depth.decremented().unwrap())),
                search.score(),
                "{} at {}", board, depth
            );
        });
    }

    #[test]
    fn one_deepen_scores_the_same_with_a_fresh_table_a_warmed_table_or_none_and_the_warmed_table_saves_nodes()
     {
        proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards())| {
            prop_assume!(!board.legal_moves().is_empty());
            let without = Fixture::deepened_once_more(board, &mut TranspositionTable::new(&mut []));
            let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
            let mut table = TranspositionTable::new(&mut store);
            let fresh = Fixture::deepened_once_more(board, &mut table);
            let warmed = Fixture::deepened_once_more(board, &mut table);
            prop_assert_eq!((fresh.score(), warmed.score()), (without.score(), without.score()), "{}", board);
            prop_assert!(warmed.nodes() < fresh.nodes(), "{}: {} vs {}", board, warmed.nodes(), fresh.nodes());
        });
    }
}
