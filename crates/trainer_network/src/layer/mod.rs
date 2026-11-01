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
    #[must_use]
    pub fn saved_as<P: Part>(self) -> &'static str {
        P::saved_name(self)
    }
}
