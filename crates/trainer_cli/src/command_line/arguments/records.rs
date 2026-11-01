use std::path::Path;

use super::Arguments;
use super::argument_file::ArgumentFile;

pub struct Records;

impl ArgumentFile for Records {
    fn pick(arguments: &Arguments) -> &Path {
        &arguments.records
    }
}
