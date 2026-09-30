#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RepetitionCount(usize);

impl RepetitionCount {
    pub const fn new(occurrences: usize) -> RepetitionCount {
        RepetitionCount(occurrences)
    }
}
