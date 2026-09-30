use arena::{ArenaError, Entrant, Opponent};
use uci::Elo;

use super::arguments::Contestant;

pub struct LimitedContestant<'arguments> {
    contestant: &'arguments Contestant,
    limit_elo: Elo,
}

impl<'arguments> LimitedContestant<'arguments> {
    pub const fn new(contestant: &'arguments Contestant, limit_elo: Elo) -> Self {
        LimitedContestant {
            contestant,
            limit_elo,
        }
    }
}

impl Entrant for LimitedContestant<'_> {
    fn opponent(&self) -> Result<Box<dyn Opponent>, ArenaError> {
        self.contestant.opponent(self.limit_elo)
    }
}
