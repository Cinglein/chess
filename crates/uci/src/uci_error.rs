use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum UciError {
    #[error("unknown command")]
    Command,
    #[error("position needs startpos or fen")]
    Position,
    #[error("setoption needs a name and a value")]
    EngineOption,
    #[error("bestmove needs a move in long algebraic notation")]
    Move,
}
