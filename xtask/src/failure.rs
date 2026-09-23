use std::io;

use thiserror::Error;

use crate::report::Report;
use crate::site::Site;

#[derive(Debug, Error)]
pub enum Failure {
    #[error("{0}")]
    Usage(String),
    #[error("cargo {0} failed")]
    Cargo(String),
    #[error("{site}: {error}")]
    Io { site: Site, error: io::Error },
    #[error("{site}: {error}")]
    Parse { site: Site, error: syn::Error },
    #[error("{}", .0.iter().map(ToString::to_string).collect::<Vec<String>>().join("\n"))]
    Lint(Vec<Report>),
}
