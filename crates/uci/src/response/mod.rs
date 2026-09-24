mod identity;
mod search_info;

pub use identity::Identity;
pub use search_info::SearchInfo;

use core::fmt;

use board::LongAlgebraic;

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Response<'line> {
    Id(Identity<'line>),
    UciOk,
    ReadyOk,
    Info(SearchInfo),
    BestMove(Option<LongAlgebraic>),
}

impl Response<'_> {
    fn best_move(rest: &str) -> Result<Self, UciError> {
        match rest.split_whitespace().next().unwrap_or("") {
            "0000" | "(none)" => Ok(Response::BestMove(None)),
            text => text
                .parse()
                .map(|notation| Response::BestMove(Some(notation)))
                .map_err(|_| UciError::UnknownMove),
        }
    }
}

impl<'line> TryFrom<&'line str> for Response<'line> {
    type Error = UciError;

    fn try_from(line: &'line str) -> Result<Response<'line>, UciError> {
        let trimmed = line.trim();
        let (head, rest) = trimmed
            .split_once(char::is_whitespace)
            .unwrap_or((trimmed, ""));
        match head {
            "id" => Identity::try_from(rest).map(Response::Id),
            "uciok" => Ok(Response::UciOk),
            "readyok" => Ok(Response::ReadyOk),
            "info" => SearchInfo::try_from(rest).map(Response::Info),
            "bestmove" => Self::best_move(rest),
            _ => Err(UciError::UnknownResponse),
        }
    }
}

impl fmt::Display for Response<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Id(identity) => write!(formatter, "id {identity}"),
            Response::UciOk => formatter.write_str("uciok"),
            Response::ReadyOk => formatter.write_str("readyok"),
            Response::Info(info) => write!(formatter, "info {info}"),
            Response::BestMove(Some(notation)) => write!(formatter, "bestmove {notation}"),
            Response::BestMove(None) => formatter.write_str("bestmove 0000"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Response;
    use crate::uci_error::UciError;

    const OWN_LINES: [&str; 7] = [
        "id name chess",
        "id author Cinglein",
        "uciok",
        "readyok",
        "info depth 5 score mate 1 nodes 33 nps 33000 time 1 pv a1a8",
        "info depth 4 score cp -50 nodes 0 nps 0 time 0",
        "bestmove e7e8q",
    ];
    const STOCKFISH_INFO: &str =
        "info depth 1 seldepth 0 multipv 1 score cp 0 nodes 0 nps 0 hashfull 0 tbhits 0 time 1 pv ";
    const STOCKFISH_INFO_KEPT: &str = "info depth 1 score cp 0 nodes 0 nps 0 time 1";
    const REJECTED: [(&str, UciError); 3] = [
        ("info string Using 1 thread", UciError::IncompleteInfo),
        (
            "Stockfish 19 by the Stockfish developers",
            UciError::UnknownResponse,
        ),
        ("bestmove zz", UciError::UnknownMove),
    ];

    #[test]
    fn every_response_prints_back_to_the_line_it_was_parsed_from() {
        for line in OWN_LINES {
            assert_eq!(Response::try_from(line).unwrap().to_string(), line);
        }
    }

    #[test]
    fn foreign_info_keys_are_skipped_and_lines_without_a_search_are_rejected() {
        let kept = Response::try_from(STOCKFISH_INFO).unwrap().to_string();
        assert_eq!(kept, STOCKFISH_INFO_KEPT);
        assert_eq!(
            Response::try_from("bestmove (none) ponder e7e5"),
            Ok(Response::BestMove(None))
        );
        for (line, error) in REJECTED {
            assert_eq!(Response::try_from(line), Err(error), "{line}");
        }
    }
}
