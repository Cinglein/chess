mod info_builder;
mod info_key;

use core::fmt;
use core::time::Duration;

use board::LongAlgebraic;
use eval::{Evaluator, Score};
use info_builder::InfoBuilder;
use info_key::InfoKey;
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
        if rest
            .split_whitespace()
            .next()
            .and_then(|head| head.parse::<InfoKey>().ok())
            == Some(InfoKey::String)
        {
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
        write!(
            formatter,
            "{} {} {} ",
            InfoKey::Depth,
            self.depth,
            InfoKey::Score
        )?;
        match self.score.mate_in_moves() {
            Some(moves) => write!(formatter, "{} {moves}", InfoKey::Mate)?,
            None => write!(formatter, "{} {}", InfoKey::Cp, self.score)?,
        }
        write!(
            formatter,
            " {} {} {} {} {} {millis}",
            InfoKey::Nodes,
            self.nodes,
            InfoKey::Nps,
            self.nodes.saturating_mul(1000) / millis.max(1),
            InfoKey::Time
        )?;
        self.best_move.map_or(Ok(()), |notation| {
            write!(formatter, " {} {notation}", InfoKey::Pv)
        })
    }
}
