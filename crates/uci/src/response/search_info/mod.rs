mod info_builder;
mod info_key;

use core::fmt;
use core::time::Duration;

use board::{LongAlgebraic, NodeCount};
use eval::{Evaluator, Score};
use info_builder::InfoBuilder;
use info_key::InfoKey;
use search::{Depth, Search};

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInfo {
    depth: Depth,
    score: Score,
    nodes: NodeCount,
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
            self.nodes.count().saturating_mul(1000) / millis.max(1),
            InfoKey::Time
        )?;
        self.best_move.map_or(Ok(()), |notation| {
            write!(formatter, " {} {notation}", InfoKey::Pv)
        })
    }
}

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for SearchInfo {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<SearchInfo>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        let score = proptest::prop_oneof![
            (-29_999i32..=29_999).prop_map(Score::new),
            (1i32..=100).prop_map(Score::mating_in_moves),
            (-100i32..=-1).prop_map(Score::mating_in_moves),
        ];
        (
            proptest::arbitrary::any::<Depth>(),
            score,
            proptest::arbitrary::any::<NodeCount>(),
            proptest::arbitrary::any::<u32>(),
            proptest::arbitrary::any::<Option<LongAlgebraic>>(),
        )
            .prop_map(|(depth, score, nodes, millis, best_move)| SearchInfo {
                depth,
                score,
                nodes,
                elapsed: Duration::from_millis(u64::from(millis)),
                best_move,
            })
            .boxed()
    }
}
