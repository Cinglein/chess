mod scope;
mod wasm;

use std::error::Error;

use strum::EnumString;

use magics::Magics;
use scope::Scope;
use task::Workspace;
use wasm::Wasm;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum ChessTask {
    Magics,
    Wasm,
}

impl ChessTask {
    pub fn run(self, flags: &[String]) -> Result<(), Box<dyn Error>> {
        let workspace = Workspace::locate()?;
        match self {
            ChessTask::Magics => Ok(Magics::run(&workspace)?),
            ChessTask::Wasm => Ok(Wasm::run(
                &workspace,
                &Scope::selection(&workspace, flags)?,
            )?),
        }
    }
}
