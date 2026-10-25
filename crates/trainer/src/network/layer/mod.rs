mod biases;
mod part;
mod weights;

pub use biases::Biases;
pub use part::Part;
pub use weights::Weights;

use strum::{AsRefStr, EnumIter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr, EnumIter)]
#[repr(u8)]
pub enum Layer {
    #[strum(serialize = "l0")]
    Hidden,
    #[strum(serialize = "l1")]
    Output,
}

impl Layer {
    pub fn saved_as<P: Part>(self) -> &'static str {
        P::saved_name(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Biases, Layer, Weights};
    use strum::IntoEnumIterator;
    #[test]
    fn weight_and_bias_ids_are_the_layer_name_with_bullets_suffixes() {
        assert!(Layer::iter().all(|layer| {
            layer.saved_as::<Weights>().strip_suffix('w') == Some(layer.as_ref())
                && layer.saved_as::<Biases>().strip_suffix('b') == Some(layer.as_ref())
        }));
    }
}
