use super::Hand;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Empty;

impl Hand for Empty {}
