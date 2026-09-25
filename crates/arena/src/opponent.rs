use board::LongAlgebraic;
use uci::{GoLimits, Position};

use crate::arena_error::ArenaError;

pub trait Opponent {
    fn begin_game(&mut self) -> Result<(), ArenaError>;

    fn choose_move(
        &mut self,
        position: Position<'_>,
        limits: GoLimits,
    ) -> Result<LongAlgebraic, ArenaError>;
}
