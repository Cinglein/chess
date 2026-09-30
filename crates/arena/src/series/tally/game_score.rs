use board::Color;

use crate::game::Verdict;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::series::tally) enum GameScore {
    Win,
    Draw,
    Loss,
}

impl GameScore {
    pub(in crate::series::tally) fn new(verdict: Verdict, challenger: Color) -> GameScore {
        match verdict.winner() {
            Some(side) if side == challenger => GameScore::Win,
            Some(_) => GameScore::Loss,
            None => GameScore::Draw,
        }
    }
}
