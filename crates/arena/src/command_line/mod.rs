mod arguments;
mod command_line_error;
mod limited_contestant;
mod settings;

pub use command_line_error::CommandLineError;

use arena::Series;
use arguments::Arguments;
use clap::Parser;
use limited_contestant::LimitedContestant;
use settings::Settings;

pub struct CommandLine;

impl CommandLine {
    pub fn play() -> Result<(), CommandLineError> {
        let arguments = Arguments::parse();
        let settings = Settings::read_or_write_defaults(arguments.config())?;
        let challenger = LimitedContestant::new(arguments.challenger(), settings.limit_elo());
        let reference = LimitedContestant::new(arguments.reference(), settings.limit_elo());
        let tally = Series::new(&challenger, &reference, settings.rules()).play(
            settings.rounds(),
            settings.concurrency(),
            &|finished| println!("{finished}"),
        )?;
        println!("{tally}");
        Ok(())
    }
}
