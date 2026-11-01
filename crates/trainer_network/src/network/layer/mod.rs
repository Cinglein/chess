mod biases;
mod part;
mod weights;

pub use biases::Biases;
pub use part::Part;
pub use weights::Weights;

use strum::AsRefStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr)]
#[repr(u8)]
pub enum Layer {
    #[strum(serialize = "l0")]
    Hidden,
    #[strum(serialize = "l1")]
    Output,
}
