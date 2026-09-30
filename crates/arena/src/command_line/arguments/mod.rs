mod contestant;

pub use contestant::Contestant;

use std::thread;

use arena::Rules;
use board::FullmoveNumber;
use clap::Parser;
use uci::Elo;

#[derive(Debug, Parser)]
#[command(
    about = "Play paired games from a set of openings at 10s+0.1s and tally them for the challenger"
)]
pub struct Arguments {
    #[arg(help = "in-process, or a path to a UCI engine")]
    challenger: Contestant,
    #[arg(help = "in-process, or a path to a UCI engine")]
    reference: Contestant,
    #[arg(
        long,
        default_value_t = 1,
        help = "Times each opening is played from both sides"
    )]
    rounds: usize,
    #[arg(
        long,
        default_value_t = Arguments::half_the_cores(),
        help = "Games played at once, each on its own pair of engines"
    )]
    concurrency: usize,
    #[arg(
        long,
        default_value_t = Contestant::STOCKFISH_FLOOR,
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
    pub const fn challenger(&self) -> &Contestant {
        &self.challenger
    }

    pub const fn reference(&self) -> &Contestant {
        &self.reference
    }

    pub const fn rounds(&self) -> usize {
        self.rounds
    }

    pub const fn concurrency(&self) -> usize {
        self.concurrency
    }

    fn half_the_cores() -> usize {
        thread::available_parallelism().map_or(1, |cores| (cores.get() / 2).max(1))
    }

    pub const fn limit_elo(&self) -> Elo {
        self.limit_elo
    }

    pub const fn longest_game(&self) -> FullmoveNumber {
        self.longest_game
    }
}
