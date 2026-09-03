use core::fmt;

use crate::fen::Fen;

pub struct FenDisplay<'a, T: Fen>(&'a T);

impl<'a, T: Fen> FenDisplay<'a, T> {
    #[must_use]
    pub const fn new(value: &'a T) -> FenDisplay<'a, T> {
        FenDisplay(value)
    }
}

impl<T: Fen> fmt::Display for FenDisplay<'_, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_fen(formatter)
    }
}
