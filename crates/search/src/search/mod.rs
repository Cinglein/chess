mod depth;
mod interrupt;
mod negamax;
mod principal;

pub use depth::Depth;
pub use interrupt::Interrupt;
pub use negamax::{TableEntry, TranspositionTable};

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
    pub const fn best_move(&self) -> Option<ChessMove> {
        self.best_move
    }

    #[must_use]
    pub const fn depth(&self) -> Depth {
        self.depth
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
        let depth = self.depth.deeper();
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
