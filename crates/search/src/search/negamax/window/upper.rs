use super::limit::Limit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Upper;

impl Limit for Upper {}
