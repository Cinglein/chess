use std::fmt;

use board::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Win(Color),
    Draw,
}

impl Verdict {
    #[must_use]
    pub const fn winner(self) -> Option<Color> {
        match self {
            Verdict::Win(side) => Some(side),
            Verdict::Draw => None,
        }
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verdict::Win(Color::White) => formatter.write_str("1-0"),
            Verdict::Win(Color::Black) => formatter.write_str("0-1"),
            Verdict::Draw => formatter.write_str("1/2-1/2"),
        }
    }
}
