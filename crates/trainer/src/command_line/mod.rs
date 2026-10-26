mod arguments;
mod command_line_error;
mod settings;

pub use command_line_error::CommandLineError;

use std::fs;

use ::settings::EmbeddedDefaults;
use arguments::{Arguments, Config, Records};
use board::Board;
use bullet_lib::value::loader::DirectSequentialDataLoader;
use clap::Parser;
use eval::SavedNetwork;
use settings::Settings;
use trainer::{Checkpoint, Network, ShuffledData};

pub struct CommandLine;

impl CommandLine {
    pub fn train() -> Result<(), CommandLineError> {
        let arguments = Arguments::parse();
        let settings = Settings::read_or_write_defaults(arguments.path::<Config>())?;
        let shuffled = ShuffledData::written_beside(arguments.path::<Records>(), arguments.seed())?;
        let path = shuffled.path().to_str().ok_or(CommandLineError::Path)?;
        Network::trainer().run(
            &settings.schedule().training_schedule(settings.net_id()),
            &settings.machine().local_settings(),
            &DirectSequentialDataLoader::new(&[path]),
        );
        let checkpoint = Checkpoint::last_of(settings.schedule(), settings.net_id());
        let bytes = fs::read(checkpoint.quantised_path(settings.machine().output_directory()))
            .map_err(CommandLineError::Checkpoint)?;
        let network = SavedNetwork::try_from(bytes.as_slice())?;
        println!(
            "{checkpoint} scores the start position at {}",
            network.evaluate(&Board::START)
        );
        Ok(())
    }
}
