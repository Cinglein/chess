use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArenaError {
    #[error("could not start the engine: {0}")]
    Spawn(io::Error),
    #[error("could not write to the engine: {0}")]
    Write(io::Error),
    #[error("the engine did not answer in time")]
    Unresponsive,
    #[error("the engine closed its output")]
    Disconnected,
    #[error("the engine had no move")]
    NoMove,
}
