mod engine_setting;

use std::path::PathBuf;

use arena::{ArenaError, InProcessEngine, Opponent, UciProcess};
use engine_setting::EngineSetting;
use strum::EnumString;
use uci::EngineOption;

#[derive(Clone, Debug, PartialEq, Eq, EnumString)]
#[repr(u8)]
pub enum Contestant {
    #[strum(serialize = "in-process")]
    InProcess,
    #[strum(default)]
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
