use game::ArenaError;
use game::Opponent;

pub trait Entrant: Sync {
    fn opponent(&self) -> Result<Box<dyn Opponent>, ArenaError>;
}
