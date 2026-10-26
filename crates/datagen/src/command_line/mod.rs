mod arguments;
mod command_line_error;
mod settings;

pub use command_line_error::CommandLineError;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use crate::{Example, Progress, SelfPlay};
use ::settings::EmbeddedDefaults;
use arena::{RandomOpenings, Rules};
use arguments::{Arguments, Config, Output};
use clap::Parser;
use settings::Settings;

pub struct CommandLine;

impl CommandLine {
    pub fn generate() -> Result<(), CommandLineError> {
        let arguments = Arguments::parse();
        let settings = Settings::read_or_write_defaults(arguments.path::<Config>())?;
        let mut output = BufWriter::new(
            File::create(arguments.path::<Output>()).map_err(CommandLineError::Output)?,
        );
        let games = SelfPlay::new(
            RandomOpenings::seeded(arguments.seed(), settings.opening_plies()),
            Rules::at_fixed_nodes(settings.nodes_per_move()),
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
