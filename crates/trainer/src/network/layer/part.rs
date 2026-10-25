use super::Layer;

pub trait Part {
    fn saved_name(layer: Layer) -> &'static str;
}
