mod identity;
mod search_info;

pub use identity::Identity;
pub use search_info::SearchInfo;

use core::fmt;

use board::ChessMove;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Response {
    Id(Identity),
    UciOk,
    ReadyOk,
    Info(SearchInfo),
    BestMove(Option<ChessMove>),
}

impl fmt::Display for Response {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Id(identity) => write!(formatter, "id {identity}"),
            Response::UciOk => formatter.write_str("uciok"),
            Response::ReadyOk => formatter.write_str("readyok"),
            Response::Info(info) => write!(formatter, "info {info}"),
            Response::BestMove(Some(chess_move)) => write!(formatter, "bestmove {chess_move}"),
            Response::BestMove(None) => formatter.write_str("bestmove 0000"),
        }
    }
}
