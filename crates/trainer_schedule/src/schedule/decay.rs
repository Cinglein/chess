use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Decay(f32);

impl Decay {
    #[must_use]
    pub const fn factor(self) -> f32 {
        self.0
    }
}
