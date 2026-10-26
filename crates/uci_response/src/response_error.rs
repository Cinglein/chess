use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum ResponseError {
    #[error("unknown response")]
    UnknownResponse,
    #[error("info needs a depth and a score")]
    IncompleteInfo,
    #[error("bestmove needs a move in long algebraic notation")]
    UnknownMove,
}
