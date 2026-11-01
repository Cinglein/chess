use super::Layer;
use super::part::Part;

pub struct Weights;

impl Part for Weights {
    fn saved_name(layer: Layer) -> &'static str {
        match layer {
            Layer::Hidden => "l0w",
            Layer::Output => "l1w",
        }
    }
}
