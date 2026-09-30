mod arguments;

use arena::{ArenaError, Rules, Series};
use arguments::Arguments;
use clap::Parser;

pub struct CommandLine;

impl CommandLine {
    pub fn play() -> Result<(), ArenaError> {
        let arguments = Arguments::parse();
        let challenger = arguments.challenger().opponent(arguments.limit_elo())?;
        let reference = arguments.reference().opponent(arguments.limit_elo())?;
        let rules = Rules::DEFAULT.lasting_at_most(arguments.longest_game());
        let tally = Series::new(challenger, reference, rules)
            .play(arguments.rounds(), &mut |finished| println!("{finished}"));
        println!("{tally}");
        Ok(())
    }
}
