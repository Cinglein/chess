use crate::arena_error::ArenaError;
use crate::opponent::Opponent;

pub trait Entrant: Sync {
    fn opponent(&self) -> Result<Box<dyn Opponent>, ArenaError>;
}
