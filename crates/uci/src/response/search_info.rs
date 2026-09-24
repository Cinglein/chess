use core::fmt;
use core::time::Duration;

use board::ChessMove;
use eval::{Evaluator, Score};
use search::{Depth, Search};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInfo {
    depth: Depth,
    score: Score,
    nodes: u64,
    elapsed: Duration,
    best_move: Option<ChessMove>,
}

impl SearchInfo {
    #[must_use]
    pub fn from_search<E: Evaluator>(search: &Search<E>, elapsed: Duration) -> SearchInfo {
        SearchInfo {
            depth: search.depth(),
            score: search.score(),
            nodes: search.nodes(),
            elapsed,
            best_move: search.best_move(),
        }
    }
}

impl fmt::Display for SearchInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let millis = u64::try_from(self.elapsed.as_millis()).unwrap_or(u64::MAX);
        write!(formatter, "depth {} score ", self.depth)?;
        match self.score.mate_in_moves() {
            Some(moves) => write!(formatter, "mate {moves}")?,
            None => write!(formatter, "cp {}", self.score)?,
        }
        write!(
            formatter,
            " nodes {} nps {} time {millis}",
            self.nodes,
            self.nodes.saturating_mul(1000) / millis.max(1)
        )?;
        self.best_move
            .map_or(Ok(()), |chess_move| write!(formatter, " pv {chess_move}"))
    }
}
