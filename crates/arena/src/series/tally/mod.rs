mod game_score;
mod pentanomial;

use std::fmt;

use board::Color;
use game_score::GameScore;
use pentanomial::{PairScore, Pentanomial};

use crate::game::Verdict;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    wins: usize,
    draws: usize,
    losses: usize,
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
    pub const fn games(&self) -> usize {
        self.wins + self.draws + self.losses
    }

    const fn counted(self, score: GameScore) -> Tally {
        match score {
            GameScore::Win => Tally {
                wins: self.wins + 1,
                ..self
            },
            GameScore::Draw => Tally {
                draws: self.draws + 1,
                ..self
            },
            GameScore::Loss => Tally {
                losses: self.losses + 1,
                ..self
            },
        }
    }
}

impl fmt::Display for Tally {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let half_points = 2 * self.wins + self.draws;
        write!(
            formatter,
            "+{} ={} -{} {}",
            self.wins,
            self.draws,
            self.losses,
            half_points / 2
        )?;
        if half_points % 2 == 1 {
            formatter.write_str(".5")?;
        }
        write!(formatter, "/{} {}", self.games(), self.pairs.estimate())
    }
}

#[cfg(test)]
mod tests {
    use board::Color;

    use super::Tally;
    use crate::game::Verdict;

    const AS_WHITE: Verdict = Verdict::Win(Color::White);
    const AS_BLACK: Verdict = Verdict::Win(Color::Black);
    const ROUND: &str = "+3 =1 -0 3.5/4";

    #[test]
    fn a_tally_counts_from_the_challenger_side_and_prints_half_points() {
        let tally = Tally::default()
            .recorded_pair(AS_WHITE, AS_BLACK)
            .recorded_pair(AS_WHITE, Verdict::Draw);
        assert!(tally.to_string().starts_with(ROUND), "{tally}");
    }
}
