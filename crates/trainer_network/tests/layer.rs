use strum::IntoEnumIterator;
use trainer_network::{Biases, Layer, Weights};
#[test]
fn weight_and_bias_ids_are_the_layer_name_with_bullets_suffixes() {
    assert!(Layer::iter().all(|layer| {
        layer.saved_as::<Weights>().strip_suffix('w') == Some(layer.as_ref())
            && layer.saved_as::<Biases>().strip_suffix('b') == Some(layer.as_ref())
    }));
}
