mod arguments;
mod command_line_error;
mod settings;

pub use command_line_error::CommandLineError;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use arena::RandomOpenings;
use arguments::Arguments;
use clap::Parser;
use datagen::{Example, Progress, SelfPlay};
use settings::Settings;

pub struct CommandLine;

impl CommandLine {
    pub fn generate() -> Result<(), CommandLineError> {
        let arguments = Arguments::parse();
        let settings = Settings::read_or_write_defaults(arguments.config())?;
        let mut output =
            BufWriter::new(File::create(arguments.output()).map_err(CommandLineError::Output)?);
        let games = SelfPlay::new(
            RandomOpenings::seeded(arguments.seed(), settings.opening_plies()),
            settings.rules(),
            settings.positions(),
        )
        .play(settings.concurrency());
        let progress = games.iter().try_fold(
            Progress::towards(settings.positions()),
            |progress, finished| -> Result<Progress, CommandLineError> {
                let written = Example::write_labels(finished.labels(), &mut output)?;
                Ok(Self::reported(progress.advanced(written)))
            },
        )?;
        output.flush().map_err(CommandLineError::Output)?;
        eprintln!("{progress}");
        Ok(())
    }

    fn reported(progress: Progress) -> Progress {
        let now = Instant::now();
        if progress.is_due(now) {
            eprintln!("{progress}");
            progress.acknowledged(now)
        } else {
            progress
        }
    }
}
