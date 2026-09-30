use std::path::PathBuf;

use arena::{ArenaError, InProcessEngine, Opponent, UciProcess};
use strum::EnumString;
use uci::{Elo, EngineOption, Switch};

#[derive(Clone, Debug, PartialEq, Eq, EnumString)]
#[repr(u8)]
pub enum Contestant {
    #[strum(serialize = "in-process")]
    InProcess,
    #[strum(default)]
    Program(PathBuf),
}

impl Contestant {
    pub const STOCKFISH_FLOOR: Elo = Elo::new(1320);

    pub fn opponent(&self, limit_elo: Elo) -> Result<Box<dyn Opponent>, ArenaError> {
        match self {
            Contestant::InProcess => Ok(Box::new(InProcessEngine::default())),
            Contestant::Program(path) => {
                let options = [
                    EngineOption::LimitStrength(Switch::True),
                    EngineOption::Elo(limit_elo),
                ];
                Ok(Box::new(UciProcess::spawn(path, &options)?))
            }
        }
    }
}
