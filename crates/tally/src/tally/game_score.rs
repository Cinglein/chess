use board::Color;

use crate::verdict::Verdict;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tally) enum GameScore {
    Win,
    Draw,
    Loss,
}

impl GameScore {
    pub(in crate::tally) fn new(verdict: Verdict, challenger: Color) -> GameScore {
        match verdict.winner() {
            Some(side) if side == challenger => GameScore::Win,
            Some(_) => GameScore::Loss,
            None => GameScore::Draw,
        }
    }
}
