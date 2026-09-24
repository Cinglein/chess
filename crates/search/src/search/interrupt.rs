pub trait Interrupt {
    fn should_stop(&self, nodes: u64) -> bool;
}
