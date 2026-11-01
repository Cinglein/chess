use newtype::Newtype;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Newtype)]
pub struct RepetitionCount(usize);
