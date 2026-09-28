mod contestant;

use clap::Parser;
use contestant::Contestant;

#[derive(Debug, Parser)]
#[command(about = "Play one game between two opponents at 10s+0.1s")]
pub struct Arguments {
    #[arg(help = "in-process, or a path to a UCI engine limited to Elo 1320")]
    white: Contestant,
    #[arg(help = "in-process, or a path to a UCI engine limited to Elo 1320")]
    black: Contestant,
}

impl Arguments {
    pub const fn white(&self) -> &Contestant {
        &self.white
    }

    pub const fn black(&self) -> &Contestant {
        &self.black
    }
}
