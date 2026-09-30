use board::NodeCount;

pub trait Interrupt {
    fn should_stop(&self, nodes: NodeCount) -> bool;
}
