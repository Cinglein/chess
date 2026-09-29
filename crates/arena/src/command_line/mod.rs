mod arguments;

use arena::{ArenaError, Game, Rules};
use arguments::Arguments;
use board::Board;
use clap::Parser;

pub struct CommandLine;

impl CommandLine {
    pub fn play() -> Result<(), ArenaError> {
        let arguments = Arguments::parse();
        let mut white = arguments.white().opponent(arguments.limit_elo())?;
        let mut black = arguments.black().opponent(arguments.limit_elo())?;
        let finished = Game::from(Board::START)
            .ruled_by(Rules::DEFAULT.lasting_at_most(arguments.longest_game()))
            .play(white.as_mut(), black.as_mut());
        println!("{finished}");
        Ok(())
    }
}
