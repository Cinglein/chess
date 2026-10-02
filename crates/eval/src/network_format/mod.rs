mod eval_scale;
mod quantisation;

pub use eval_scale::EvalScale;
pub use quantisation::Quantisation;

use board::{Color, PieceKind, Square};
use strum::EnumCount;

pub struct NetworkFormat;

impl NetworkFormat {
    pub const INPUT_SIZE: usize = Color::COUNT * PieceKind::COUNT * Square::COUNT;
    pub const HIDDEN_SIZE: usize = 128;
    pub const OUTPUT_SIZE: usize = 1;
    pub const HIDDEN_QUANTISATION: Quantisation = Quantisation::new(255);
    pub const OUTPUT_QUANTISATION: Quantisation = Quantisation::new(64);
    pub const OUTPUT_BIAS_QUANTISATION: Quantisation =
        Self::HIDDEN_QUANTISATION.times(Self::OUTPUT_QUANTISATION);
    pub const EVAL_SCALE: EvalScale = EvalScale::new(400);
}
