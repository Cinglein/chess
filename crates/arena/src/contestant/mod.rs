mod contestant_word;
mod engine_setting;

use std::path::PathBuf;

use arena::{ArenaError, InProcessEngine, Opponent, UciProcess};
use contestant_word::ContestantWord;
use engine_setting::EngineSetting;
use uci::EngineOption;

pub enum Contestant {
    InProcess,
    Program(PathBuf),
}

impl Contestant {
    const ANCHOR_ELO: u16 = 1320;

    pub fn opponent(&self) -> Result<Box<dyn Opponent>, ArenaError> {
        match self {
            Contestant::InProcess => Ok(Box::new(InProcessEngine::default())),
            Contestant::Program(path) => {
                let limited = true.to_string();
                let elo = Self::ANCHOR_ELO.to_string();
                let options = [
                    EngineOption::new(EngineSetting::LimitStrength.into(), &limited),
                    EngineOption::new(EngineSetting::Elo.into(), &elo),
                ];
                Ok(Box::new(UciProcess::spawn(path, &options)?))
            }
        }
    }
}

impl From<&str> for Contestant {
    fn from(text: &str) -> Contestant {
        match text.parse::<ContestantWord>() {
            Ok(ContestantWord::InProcess) => Contestant::InProcess,
            Err(_) => Contestant::Program(PathBuf::from(text)),
        }
    }
}
