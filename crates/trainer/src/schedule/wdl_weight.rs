use bullet_lib::trainer::schedule::wdl::ConstantWDL;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Display, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WdlWeight(f32);

impl WdlWeight {
    #[must_use]
    pub const fn constant(self) -> ConstantWDL {
        ConstantWDL { value: self.0 }
    }
}
