use board::Color;

use crate::game::Verdict;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GameScore {
    Win,
    Draw,
    Loss,
}

impl GameScore {
    pub(super) fn new(verdict: Verdict, challenger: Color) -> GameScore {
        match verdict.winner() {
            Some(side) if side == challenger => GameScore::Win,
            Some(_) => GameScore::Loss,
            None => GameScore::Draw,
        }
    }
}
