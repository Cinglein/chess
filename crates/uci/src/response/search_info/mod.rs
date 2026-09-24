mod info_builder;

use core::fmt;
use core::time::Duration;

use board::LongAlgebraic;
use eval::{Evaluator, Score};
use info_builder::InfoBuilder;
use search::{Depth, Search};

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInfo {
    depth: Depth,
    score: Score,
    nodes: u64,
    elapsed: Duration,
    best_move: Option<LongAlgebraic>,
}

impl SearchInfo {
    #[must_use]
    pub fn from_search<E: Evaluator>(search: &Search<E>, elapsed: Duration) -> SearchInfo {
        SearchInfo {
            depth: search.depth(),
            score: search.score(),
            nodes: search.nodes(),
            elapsed,
            best_move: search.best_move().map(LongAlgebraic::from),
        }
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
    pub const fn best_move(&self) -> Option<LongAlgebraic> {
        self.best_move
    }
}

impl TryFrom<&str> for SearchInfo {
    type Error = UciError;

    fn try_from(rest: &str) -> Result<SearchInfo, UciError> {
        if rest.trim_start().starts_with("string") {
            return Err(UciError::IncompleteInfo);
        }
        rest.split_whitespace()
            .fold(InfoBuilder::default(), InfoBuilder::absorb)
            .info()
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
            .map_or(Ok(()), |notation| write!(formatter, " pv {notation}"))
    }
}
