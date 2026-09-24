use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum UciError {
    #[error("unknown command")]
    UnknownCommand,
    #[error("position needs startpos or fen")]
    UnknownPosition,
}
