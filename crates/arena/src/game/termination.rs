use derive_more::Display;

use crate::arena_error::ArenaError;

#[derive(Debug, Display)]
pub enum Termination {
    #[display("checkmate")]
    Checkmate,
    #[display("stalemate")]
    Stalemate,
    #[display("fifty moves without progress")]
    FiftyMoves,
    #[display("threefold repetition")]
    Repetition,
    #[display("insufficient material")]
    InsufficientMaterial,
    #[display("time forfeit")]
    TimeForfeit,
    #[display("illegal move")]
    IllegalMove,
    #[display("move limit")]
    MoveLimit,
    #[display("resignation adjudicated")]
    Resignation,
    #[display("draw adjudicated")]
    DrawAdjudicated,
    #[display("engine failure: {_0}")]
    Failure(ArenaError),
}

impl Termination {
    #[must_use]
    pub const fn defeats_the_side_to_move(&self) -> bool {
        matches!(
            self,
            Termination::Checkmate
                | Termination::Resignation
                | Termination::TimeForfeit
                | Termination::IllegalMove
                | Termination::Failure(_)
        )
    }
}
