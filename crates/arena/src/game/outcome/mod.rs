mod verdict;

pub use verdict::Verdict;

use std::fmt;

use board::Color;

use super::termination::Termination;

#[derive(Debug)]
pub struct Outcome {
    verdict: Verdict,
    termination: Termination,
}

impl Outcome {
    #[must_use]
    pub fn new(side_to_move: Color, termination: Termination) -> Outcome {
        Outcome {
            verdict: if termination.defeats_the_side_to_move() {
                Verdict::Win(!side_to_move)
            } else {
                Verdict::Draw
            },
            termination,
        }
    }

    #[must_use]
    pub const fn verdict(&self) -> Verdict {
        self.verdict
    }

    #[must_use]
    pub const fn termination(&self) -> &Termination {
        &self.termination
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {{{}}}", self.verdict, self.termination)
    }
}
