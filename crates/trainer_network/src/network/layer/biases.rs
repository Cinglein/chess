use super::Layer;
use super::part::Part;

pub struct Biases;

impl Part for Biases {
    fn saved_name(layer: Layer) -> &'static str {
        match layer {
            Layer::Hidden => "l0b",
            Layer::Output => "l1b",
        }
    }
}
