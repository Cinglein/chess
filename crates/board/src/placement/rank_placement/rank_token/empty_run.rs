#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::placement::rank_placement) struct EmptyRun(u8);

impl EmptyRun {
    pub(super) const fn length(self) -> usize {
        self.0 as usize
    }
}

impl From<u8> for EmptyRun {
    fn from(squares: u8) -> EmptyRun {
        EmptyRun(squares)
    }
}
