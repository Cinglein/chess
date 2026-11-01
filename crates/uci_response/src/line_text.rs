use derive_more::Display;
use newtype::Newtype;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Newtype)]
pub struct LineText<'line>(&'line str);
