use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Site {
    Workspace,
    File(String),
    Line(String, usize),
}

impl fmt::Display for Site {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Site::Workspace => formatter.write_str("workspace"),
            Site::File(path) => formatter.write_str(path),
            Site::Line(path, line) => write!(formatter, "{path}:{line}"),
        }
    }
}
