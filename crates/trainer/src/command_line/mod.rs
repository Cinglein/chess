mod arguments;
mod command_line_error;
mod settings;

pub use command_line_error::CommandLineError;

use arguments::Arguments;
use bullet_lib::value::loader::DirectSequentialDataLoader;
use clap::Parser;
use settings::Settings;
use trainer::{Network, ShuffledData};

pub struct CommandLine;

impl CommandLine {
    pub fn train() -> Result<(), CommandLineError> {
        let arguments = Arguments::parse();
        let settings = Settings::read_or_write_defaults(arguments.config())?;
        let shuffled = ShuffledData::written_beside(arguments.records(), arguments.seed())?;
        let path = shuffled.path().to_str().ok_or(CommandLineError::Path)?;
        Network::trainer().run(
            &settings.schedule().training_schedule(settings.net_id()),
            &settings.machine().local_settings(),
            &DirectSequentialDataLoader::new(&[path]),
        );
        Ok(())
    }
}
