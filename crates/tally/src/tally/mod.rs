mod game_count;
mod game_score;
mod pentanomial;

use std::fmt;

use board::Color;
use game_count::GameCount;
use game_score::GameScore;
use pentanomial::{PairScore, Pentanomial};

use crate::verdict::Verdict;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    wins: GameCount,
    draws: GameCount,
    losses: GameCount,
    pairs: Pentanomial,
}

impl Tally {
    #[must_use]
    pub fn recorded_pair(self, as_white: Verdict, as_black: Verdict) -> Tally {
        let white = GameScore::new(as_white, Color::White);
        let black = GameScore::new(as_black, Color::Black);
        Tally {
            pairs: self.pairs.counted(PairScore::new(white, black)),
            ..self.counted(white).counted(black)
        }
    }

    #[must_use]
    pub fn merged(self, other: Tally) -> Tally {
        Tally {
            wins: self.wins + other.wins,
            draws: self.draws + other.draws,
            losses: self.losses + other.losses,
            pairs: self.pairs.merged(other.pairs),
        }
    }

    #[must_use]
    pub fn games(&self) -> GameCount {
        self.wins + self.draws + self.losses
    }

    const fn counted(self, score: GameScore) -> Tally {
        match score {
            GameScore::Win => Tally {
                wins: self.wins.incremented(),
                ..self
            },
            GameScore::Draw => Tally {
                draws: self.draws.incremented(),
                ..self
            },
            GameScore::Loss => Tally {
                losses: self.losses.incremented(),
                ..self
            },
        }
    }
}

impl fmt::Display for Tally {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "+{} ={} -{} {}/{} {}",
            self.wins,
            self.draws,
            self.losses,
            self.wins.half_points(self.draws),
            self.games(),
            self.pairs.estimate()
        )
    }
}
