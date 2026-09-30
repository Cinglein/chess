use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct MovesToGo(u32);

impl MovesToGo {
    pub(super) const fn new(moves: u32) -> MovesToGo {
        MovesToGo(moves)
    }

    pub(super) fn share_of(self, remaining: Duration) -> Duration {
        remaining / self.0
    }
}
