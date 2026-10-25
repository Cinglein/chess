use board::{Board, NodeCount};

use crate::arena_error::ArenaError;
use crate::entrant::Entrant;
use crate::game::{Finished, Game};
use crate::in_process_engine::InProcessEngine;
use crate::opponent::Opponent;
use crate::rules::Rules;

pub const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
pub const QUICK: Rules = Rules::at_fixed_nodes(NodeCount::new(64));

pub struct InProcess;

impl InProcess {
    pub fn played_between_two_engines(fen: &str, rules: Rules) -> Finished {
        let board: Board = fen.parse().unwrap();
        Game::from(board).ruled_by(rules).play(
            &mut InProcessEngine::default(),
            &mut InProcessEngine::default(),
        )
    }
}

impl Entrant for InProcess {
    fn opponent(&self) -> Result<Box<dyn Opponent>, ArenaError> {
        Ok(Box::new(InProcessEngine::default()))
    }
}
