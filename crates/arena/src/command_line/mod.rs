mod arguments;
mod command_line_error;
mod limited_contestant;
mod settings;

pub use command_line_error::CommandLineError;

use ::settings::EmbeddedDefaults;
use arena::{Openings, Rules, Series};
use arguments::{Arguments, Challenger, Reference};
use clap::Parser;
use limited_contestant::LimitedContestant;
use settings::Settings;

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
