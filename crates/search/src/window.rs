use crate::bound::Bound;
use crate::lower::Lower;
use crate::upper::Upper;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Window {
    lower: Bound<Lower>,
    upper: Bound<Upper>,
}

impl Window {
    pub(crate) const FULL: Window = Window {
        lower: Bound::LOWEST,
        upper: Bound::HIGHEST,
    };

    pub(crate) const fn new(lower: Bound<Lower>, upper: Bound<Upper>) -> Window {
        Window { lower, upper }
    }

    pub(crate) const fn lower(self) -> Bound<Lower> {
        self.lower
    }

    pub(crate) const fn upper(self) -> Bound<Upper> {
        self.upper
    }

    pub(crate) const fn below(self, upper: Bound<Upper>) -> Window {
        Window { upper, ..self }
    }
}
