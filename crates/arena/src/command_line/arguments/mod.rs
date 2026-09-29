mod contestant;
mod elo;

use arena::Rules;
use board::FullmoveNumber;
use clap::Parser;
use contestant::Contestant;
use elo::Elo;

#[derive(Debug, Parser)]
#[command(about = "Play one game between two opponents at 10s+0.1s")]
pub struct Arguments {
    #[arg(help = "in-process, or a path to a UCI engine")]
    white: Contestant,
    #[arg(help = "in-process, or a path to a UCI engine")]
    black: Contestant,
    #[arg(
        long,
        default_value_t = Elo::STOCKFISH_FLOOR,
        help = "Elo that every UCI engine is limited to with UCI_LimitStrength"
    )]
    limit_elo: Elo,
    #[arg(
        long,
        default_value_t = Rules::DEFAULT.longest_game(),
        help = "Move number at which an unfinished game is adjudicated a draw"
    )]
    longest_game: FullmoveNumber,
}

impl Arguments {
    pub const fn white(&self) -> &Contestant {
        &self.white
    }

    pub const fn black(&self) -> &Contestant {
        &self.black
    }

    pub const fn limit_elo(&self) -> Elo {
        self.limit_elo
    }

    pub const fn longest_game(&self) -> FullmoveNumber {
        self.longest_game
    }
}
