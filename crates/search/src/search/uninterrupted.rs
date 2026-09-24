use super::interrupt::Interrupt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Uninterrupted;

impl Interrupt for Uninterrupted {
    fn should_stop(&self, _: u64) -> bool {
        false
    }
}
