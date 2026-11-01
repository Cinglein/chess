#![cfg_attr(not(test), no_std)]

mod identity;
mod no_move;
mod response_error;
mod response_word;
mod search_info;

pub use identity::Identity;
pub use search_info::SearchInfo;

use core::fmt;

use board::LongAlgebraic;
use no_move::NoMove;
use response_word::ResponseWord;

use crate::response_error::ResponseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Response<'line> {
    Id(Identity<'line>),
    UciOk,
    ReadyOk,
    Info(SearchInfo),
    BestMove(Option<LongAlgebraic>),
}

impl Response<'_> {
    #[must_use]
    pub const fn best_move(&self) -> Option<Option<LongAlgebraic>> {
        match self {
            Response::BestMove(notation) => Some(*notation),
            _ => None,
        }
    }

    #[must_use]
    pub const fn info(&self) -> Option<SearchInfo> {
        match self {
            Response::Info(info) => Some(*info),
            _ => None,
        }
    }

    const fn word(&self) -> ResponseWord {
        match self {
            Response::Id(_) => ResponseWord::Id,
            Response::UciOk => ResponseWord::UciOk,
            Response::ReadyOk => ResponseWord::ReadyOk,
            Response::Info(_) => ResponseWord::Info,
            Response::BestMove(_) => ResponseWord::BestMove,
        }
    }

    fn parse_best_move(rest: &str) -> Result<Self, ResponseError> {
        let text = rest.split_whitespace().next().unwrap_or("");
        match text.parse::<NoMove>() {
            Ok(_) => Ok(Response::BestMove(None)),
            Err(_) => text
                .parse()
                .map(|notation| Response::BestMove(Some(notation)))
                .map_err(|_| ResponseError::UnknownMove),
        }
    }
}

impl<'line> TryFrom<&'line str> for Response<'line> {
    type Error = ResponseError;

    fn try_from(line: &'line str) -> Result<Response<'line>, ResponseError> {
        let trimmed = line.trim();
        let (head, rest) = trimmed
            .split_once(char::is_whitespace)
            .unwrap_or((trimmed, ""));
        match head
            .parse::<ResponseWord>()
            .map_err(|_| ResponseError::UnknownResponse)?
        {
            ResponseWord::Id => Identity::try_from(rest).map(Response::Id),
            ResponseWord::UciOk => Ok(Response::UciOk),
            ResponseWord::ReadyOk => Ok(Response::ReadyOk),
            ResponseWord::Info => SearchInfo::try_from(rest).map(Response::Info),
            ResponseWord::BestMove => Self::parse_best_move(rest),
        }
    }
}

impl fmt::Display for Response<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.word())?;
        match self {
            Response::Id(identity) => write!(formatter, " {identity}"),
            Response::Info(info) => write!(formatter, " {info}"),
            Response::BestMove(Some(notation)) => write!(formatter, " {notation}"),
            Response::BestMove(None) => write!(formatter, " {}", NoMove::Zeros),
            _ => Ok(()),
        }
    }
}
