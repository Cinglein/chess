use crate::limit::Limit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Lower;

impl Limit for Lower {}
