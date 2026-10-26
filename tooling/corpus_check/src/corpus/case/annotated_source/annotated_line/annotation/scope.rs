use std::path::Path;

use strum::{Display, EnumString};

use task::LineNumber;
use task::Site;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
#[repr(u8)]
pub enum Scope {
    Line,
    File,
    Workspace,
}

impl Scope {
    pub fn site(self, path: &Path, line: LineNumber) -> Site {
        match self {
            Scope::Line => Site::Line(path.to_path_buf(), line),
            Scope::File => Site::File(path.to_path_buf()),
            Scope::Workspace => Site::Workspace,
        }
    }
}
