#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::placement::rank_placement) struct EmptyRun(u8);

impl EmptyRun {
    pub(super) const fn new(squares: u8) -> EmptyRun {
        EmptyRun(squares)
    }

    pub(super) const fn length(self) -> usize {
        self.0 as usize
    }
}
