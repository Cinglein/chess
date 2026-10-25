use std::io;

use thiserror::Error;

use crate::task::report::Report;
use crate::task::site::Site;

#[derive(Debug, Error)]
pub enum Failure {
    #[error("{0}")]
    Usage(String),
    #[error("{0} failed")]
    Command(String),
    #[error("gh failed: {0}")]
    GitHub(String),
    #[error("cargo metadata is not the shape expected: {0}")]
    Metadata(#[from] serde_json::Error),
    #[error("refused: {0}")]
    Refused(String),
    #[error("{site}: {error}")]
    Io { site: Site, error: io::Error },
    #[error("{site}: {error}")]
    Parse { site: Site, error: syn::Error },
    #[error("{}", .0.iter().map(ToString::to_string).collect::<Vec<String>>().join("\n"))]
    Lint(Vec<Report>),
}
