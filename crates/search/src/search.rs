use core::marker::PhantomData;

use board::{Board, ChessMove, State};
use eval::{Evaluator, Score};

use crate::depth::Depth;
use crate::negamax::Negamax;
use crate::principal::Principal;
use crate::root_distance::RootDistance;
use crate::transposition_table::TranspositionTable;

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
    pub fn deepen(self, table: &mut TranspositionTable<'_>) -> Search<E> {
        let depth = self.depth.incremented();
        let remaining = depth.decremented().unwrap_or_default();
        let mut negamax = Negamax::<E>::new(table);
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
        Search {
            depth,
            best_move: principal.chess_move(),
            score: principal.chess_move().map_or_else(
                || Negamax::<E>::terminal(&self.board, RootDistance::ROOT),
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
    use crate::table_entry::TableEntry;
    use crate::transposition_table::TranspositionTable;

    const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
    const STALEMATE: &str = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
    const DEFENDED_PAWN: &str = "6k1/8/4p3/3p4/8/8/8/3Q2K1 w - - 0 1";
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const ORDERING_DEPTH: u8 = 4;
    const TABLE_ENTRIES: usize = 1 << 16;

    struct Fixture;

    impl Fixture {
        fn searched(fen: &str, depth: u8, entries: usize) -> Search<PieceSquareTables> {
            let mut store = vec![TableEntry::EMPTY; entries];
            let mut table = TranspositionTable::new(&mut store);
            let board: Board = fen.parse().unwrap();
            (0..depth).fold(Search::from(board), |search, _| search.deepen(&mut table))
        }
    }

    #[test]
    fn mate_in_one_is_found_and_stalemate_has_no_move_and_a_drawn_score() {
        let search = Fixture::searched(MATE_IN_ONE, 2, TABLE_ENTRIES);
        let best = search.best_move().map(|chess_move| chess_move.to_string());
        assert_eq!(
            (best.as_deref(), search.score()),
            (Some("a1a8"), Score::mate_in(1))
        );
        let search = Fixture::searched(STALEMATE, 1, TABLE_ENTRIES);
        assert_eq!((search.best_move(), search.score()), (None, Score::DRAW));
    }

    #[test]
    fn a_defended_pawn_is_not_taken_at_depth_one_because_the_recapture_is_seen() {
        let search = Fixture::searched(DEFENDED_PAWN, 1, TABLE_ENTRIES);
        let best = search.best_move().map(|chess_move| chess_move.to_string());
        assert_ne!(best.as_deref(), Some("d1d5"));
        assert!(search.score() > Score::DRAW);
    }

    #[test]
    fn the_table_cuts_nodes_without_changing_the_score() {
        let without = Fixture::searched(KIWIPETE, ORDERING_DEPTH, 0);
        let with = Fixture::searched(KIWIPETE, ORDERING_DEPTH, TABLE_ENTRIES);
        assert_eq!(with.score(), without.score());
        assert!(
            with.nodes() < without.nodes(),
            "{} vs {}",
            with.nodes(),
            without.nodes()
        );
    }
}
