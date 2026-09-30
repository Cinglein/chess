mod arguments;
mod limited_contestant;

use arena::{ArenaError, Rules, Series};
use arguments::Arguments;
use clap::Parser;
use limited_contestant::LimitedContestant;

pub struct CommandLine;

impl CommandLine {
    pub fn play() -> Result<(), ArenaError> {
        let arguments = Arguments::parse();
        let challenger = LimitedContestant::new(arguments.challenger(), arguments.limit_elo());
        let reference = LimitedContestant::new(arguments.reference(), arguments.limit_elo());
        let rules = Rules::DEFAULT.lasting_at_most(arguments.longest_game());
        let tally = Series::new(&challenger, &reference, rules).play(
            arguments.rounds(),
            arguments.concurrency(),
            &|finished| println!("{finished}"),
        )?;
        println!("{tally}");
        Ok(())
    }
}
