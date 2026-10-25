use std::path::Path;

use super::Arguments;
use super::argument_file::ArgumentFile;

pub struct Config;

impl ArgumentFile for Config {
    fn pick(arguments: &Arguments) -> &Path {
        &arguments.config
    }
}
