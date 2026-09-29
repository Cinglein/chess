mod contestant;

use clap::Parser;
use contestant::Contestant;

#[derive(Debug, Parser)]
#[command(about = "Play one game between two opponents at 10s+0.1s")]
pub struct Arguments {
    #[arg(help = "in-process, or a path to a UCI engine")]
    white: Contestant,
    #[arg(help = "in-process, or a path to a UCI engine")]
    black: Contestant,
    #[arg(
        long,
        default_value_t = 1320,
        help = "Elo that every UCI engine is limited to with UCI_LimitStrength"
    )]
    limit_elo: u16,
    #[arg(
        long,
        default_value_t = 1000,
        help = "Plies after which an unfinished game is adjudicated a draw"
    )]
    longest_game: u16,
}

impl Arguments {
    pub const fn white(&self) -> &Contestant {
        &self.white
    }

    pub const fn black(&self) -> &Contestant {
        &self.black
    }

    pub const fn limit_elo(&self) -> u16 {
        self.limit_elo
    }

    pub const fn longest_game(&self) -> u16 {
        self.longest_game
    }
}
