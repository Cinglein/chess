use core::marker::PhantomData;

use board::{Board, ChessMove, State};
use eval::{Evaluator, Score};

use crate::depth::Depth;
use crate::negamax::Negamax;
use crate::principal::Principal;

pub struct Search<E: Evaluator> {
    board: Board,
    depth: Depth,
    best_move: Option<ChessMove>,
    score: Score,
    nodes: u64,
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
    pub const fn nodes(&self) -> u64 {
        self.nodes
    }

    #[must_use]
    pub fn deepen(self) -> Search<E> {
        let depth = self.depth.incremented();
        let remaining = depth.decremented().unwrap_or_default();
        let mut negamax = Negamax::<E>::new();
        let principal = negamax
            .root_moves(&self.board, self.best_move)
            .into_iter()
            .filter_map(|chess_move| {
                self.board
                    .make_move(chess_move)
                    .map(|child| (chess_move, child))
            })
            .fold(Principal::NONE, |principal, (chess_move, child)| {
                let score = -negamax.score(&child, remaining, 1, principal.window());
                principal.improved(chess_move, score)
            });
        Search {
            depth,
            best_move: principal.chess_move(),
            score: principal.chess_move().map_or_else(
                || Negamax::<E>::terminal(&self.board, 0),
                |_| principal.score(),
            ),
            nodes: self.nodes + negamax.nodes(),
            ..self
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
            nodes: 0,
            evaluator: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use eval::{PieceSquareTables, Score};

    use super::Search;

    const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
    const STALEMATE: &str = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
    const DEFENDED_PAWN: &str = "6k1/8/4p3/3p4/8/8/8/3Q2K1 w - - 0 1";
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const ORDERING_DEPTH: u8 = 4;

    #[test]
    fn mate_in_one_is_found_and_stalemate_has_no_move_and_a_drawn_score() {
        let mated: Board = MATE_IN_ONE.parse().unwrap();
        let search = Search::<PieceSquareTables>::from(mated).deepen().deepen();
        let best = search.best_move().map(|chess_move| chess_move.to_string());
        assert_eq!(
            (best.as_deref(), search.score()),
            (Some("a1a8"), Score::mate_in(1))
        );
        let stalemated: Board = STALEMATE.parse().unwrap();
        let search = Search::<PieceSquareTables>::from(stalemated).deepen();
        assert_eq!((search.best_move(), search.score()), (None, Score::DRAW));
    }

    #[test]
    fn a_defended_pawn_is_not_taken_at_depth_one_because_the_recapture_is_seen() {
        let board: Board = DEFENDED_PAWN.parse().unwrap();
        let search = Search::<PieceSquareTables>::from(board).deepen();
        let best = search.best_move().map(|chess_move| chess_move.to_string());
        assert_ne!(best.as_deref(), Some("d1d5"));
        assert!(search.score() > Score::DRAW);
    }

    #[test]
    fn ordered_search_visits_a_small_fraction_of_the_perft_tree() {
        let board: Board = KIWIPETE.parse().unwrap();
        let deepened = |search: Search<PieceSquareTables>, _: u8| search.deepen();
        let search = (0..ORDERING_DEPTH).fold(Search::from(board), deepened);
        let leaves = board.perft(ORDERING_DEPTH);
        assert!(
            search.nodes() * u64::from(ORDERING_DEPTH) < leaves,
            "{}",
            search.nodes()
        );
    }
}
