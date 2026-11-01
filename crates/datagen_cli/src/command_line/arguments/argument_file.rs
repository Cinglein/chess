use std::path::Path;

use super::Arguments;

pub trait ArgumentFile {
    fn pick(arguments: &Arguments) -> &Path;
}
