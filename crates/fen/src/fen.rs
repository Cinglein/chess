use core::fmt;

use crate::fen_display::FenDisplay;
use crate::fen_error::FenError;

pub trait Fen: Sized {
    fn fmt_fen(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn from_fen(text: &str) -> Result<Self, FenError>;

    fn fen(&self) -> FenDisplay<'_, Self> {
        FenDisplay::new(self)
    }
}
