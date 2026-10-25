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
    pub const fn weights(self) -> &'static str {
        match self {
            Layer::Hidden => "l0w",
            Layer::Output => "l1w",
        }
    }

    pub const fn biases(self) -> &'static str {
        match self {
            Layer::Hidden => "l0b",
            Layer::Output => "l1b",
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Layer;

    #[test]
    fn weight_and_bias_ids_are_the_layer_name_with_bullets_suffixes() {
        assert!(Layer::iter().all(|layer| {
            layer.weights().strip_suffix('w') == Some(layer.as_ref())
                && layer.biases().strip_suffix('b') == Some(layer.as_ref())
        }));
    }
}
