use std::path::Path;

use super::Arguments;
use super::argument_file::ArgumentFile;

pub struct Output;

impl ArgumentFile for Output {
    fn pick(arguments: &Arguments) -> &Path {
        &arguments.output
    }
}
