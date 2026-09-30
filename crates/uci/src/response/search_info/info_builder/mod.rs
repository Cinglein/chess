use core::time::Duration;

use board::{LongAlgebraic, NodeCount};
use eval::Score;
use search::Depth;

use super::SearchInfo;
use super::info_key::InfoKey;
use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct InfoBuilder {
    pending: Option<InfoKey>,
    depth: Option<Depth>,
    score: Option<Score>,
    nodes: Option<NodeCount>,
    elapsed: Option<Duration>,
    best_move: Option<LongAlgebraic>,
}

impl InfoBuilder {
    pub(super) fn absorb(self, token: &str) -> InfoBuilder {
        match (self.pending, token.parse::<InfoKey>()) {
            (Some(key), _) => self.assign(key, token),
            (None, Ok(InfoKey::Score | InfoKey::String) | Err(_)) => self,
            (None, Ok(key)) => InfoBuilder {
                pending: Some(key),
                ..self
            },
        }
    }

    pub(super) fn info(self) -> Result<SearchInfo, UciError> {
        Ok(SearchInfo {
            depth: self.depth.ok_or(UciError::IncompleteInfo)?,
            score: self.score.ok_or(UciError::IncompleteInfo)?,
            nodes: self.nodes.unwrap_or_default(),
            elapsed: self.elapsed.unwrap_or_default(),
            best_move: self.best_move,
        })
    }

    fn assign(self, key: InfoKey, token: &str) -> InfoBuilder {
        let cleared = InfoBuilder {
            pending: None,
            ..self
        };
        match key {
            InfoKey::Score | InfoKey::Nps | InfoKey::String => cleared,
            InfoKey::Depth => InfoBuilder {
                depth: token.parse().ok(),
                ..cleared
            },
            InfoKey::Cp => InfoBuilder {
                score: token.parse().ok(),
                ..cleared
            },
            InfoKey::Mate => InfoBuilder {
                score: token.parse::<i32>().ok().map(Score::mating_in_moves),
                ..cleared
            },
            InfoKey::Nodes => InfoBuilder {
                nodes: token.parse().ok(),
                ..cleared
            },
            InfoKey::Time => InfoBuilder {
                elapsed: token.parse::<u64>().ok().map(Duration::from_millis),
                ..cleared
            },
            InfoKey::Pv => InfoBuilder {
                best_move: token.parse().ok(),
                ..cleared
            },
        }
    }
}
