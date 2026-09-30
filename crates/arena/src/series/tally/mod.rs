mod game_score;

use std::fmt;

use board::Color;
use game_score::GameScore;

use crate::game::Verdict;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    wins: usize,
    draws: usize,
    losses: usize,
}

impl Tally {
    #[must_use]
    pub fn recorded(self, verdict: Verdict, challenger: Color) -> Tally {
        match GameScore::new(verdict, challenger) {
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

    #[must_use]
    pub const fn games(&self) -> usize {
        self.wins + self.draws + self.losses
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
        write!(formatter, "/{}", self.games())
    }
}

#[cfg(test)]
mod tests {
    use board::Color;

    use super::Tally;
    use crate::game::Verdict;

    const VERDICTS: [Verdict; 3] = [
        Verdict::Win(Color::White),
        Verdict::Draw,
        Verdict::Win(Color::Black),
    ];
    const AS_WHITE: &str = "+1 =1 -1 1.5/3";

    #[test]
    fn a_tally_counts_from_the_challenger_side_and_prints_half_points() {
        let tally = VERDICTS
            .into_iter()
            .fold(Tally::default(), |tally, verdict| {
                tally.recorded(verdict, Color::White)
            });
        assert_eq!(tally.to_string(), AS_WHITE);
    }
}
