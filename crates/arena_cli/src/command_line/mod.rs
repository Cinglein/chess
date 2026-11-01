mod arguments;
mod command_line_error;
mod limited_contestant;

pub use command_line_error::CommandLineError;

use arena::{Openings, Series, Settings};
use arguments::{Arguments, Challenger, Reference};
use clap::Parser;
use game::Rules;
use limited_contestant::LimitedContestant;
use settings::EmbeddedDefaults;

pub struct CommandLine;

impl CommandLine {
    pub fn play() -> Result<(), CommandLineError> {
        let arguments = Arguments::parse();
        let settings = Settings::read_or_write_defaults(arguments.config())?;
        let challenger =
            LimitedContestant::new(arguments.contestant::<Challenger>(), settings.limit_elo());
        let reference =
            LimitedContestant::new(arguments.contestant::<Reference>(), settings.limit_elo());
        let rules = Rules::new(
            settings.time_control(),
            settings.longest_game(),
            settings.draw_adjudication(),
            settings.resign_adjudication(),
        );
        let tally = Series::new(&challenger, &reference, rules).play(
            Openings::repeated(settings.rounds()),
            settings.concurrency(),
            &|finished| println!("{finished}"),
        )?;
        println!("{tally}");
        Ok(())
    }
}
