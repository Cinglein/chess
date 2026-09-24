use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum UciError {
    #[error("unknown command")]
    UnknownCommand,
    #[error("position needs startpos or fen")]
    UnknownPosition,
    #[error("setoption needs a name and a value")]
    UnknownOption,
    #[error("unknown response")]
    UnknownResponse,
    #[error("info needs a depth and a score")]
    IncompleteInfo,
    #[error("bestmove needs a move in long algebraic notation")]
    UnknownMove,
}
