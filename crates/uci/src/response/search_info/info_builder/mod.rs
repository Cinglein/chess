use core::time::Duration;

use board::LongAlgebraic;
use eval::Score;
use search::Depth;

use super::SearchInfo;
use super::info_key::InfoKey;
use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct InfoBuilder {
    pending: Option<InfoKey>,
    depth: Option<u8>,
    centipawns: Option<i32>,
    mate: Option<i32>,
    nodes: Option<u64>,
    millis: Option<u64>,
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
        let score = self
            .mate
            .map(Score::mating_in_moves)
            .or_else(|| self.centipawns.map(Score::new))
            .ok_or(UciError::IncompleteInfo)?;
        Ok(SearchInfo {
            depth: self.depth.map(Depth::new).ok_or(UciError::IncompleteInfo)?,
            score,
            nodes: self.nodes.unwrap_or(0),
            elapsed: Duration::from_millis(self.millis.unwrap_or(0)),
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
                centipawns: token.parse().ok(),
                ..cleared
            },
            InfoKey::Mate => InfoBuilder {
                mate: token.parse().ok(),
                ..cleared
            },
            InfoKey::Nodes => InfoBuilder {
                nodes: token.parse().ok(),
                ..cleared
            },
            InfoKey::Time => InfoBuilder {
                millis: token.parse().ok(),
                ..cleared
            },
            InfoKey::Pv => InfoBuilder {
                best_move: token.parse().ok(),
                ..cleared
            },
        }
    }
}
