use newtype::Newtype;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Newtype)]
pub(super) struct MovesToGo(u32);

impl MovesToGo {
    pub(super) fn share_of(self, remaining: Duration) -> Duration {
        remaining / self.0
    }
}
