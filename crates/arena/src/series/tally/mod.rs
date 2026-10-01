mod game_count;
mod game_score;
mod pentanomial;

use std::fmt;

use board::Color;
use game_count::GameCount;
use game_score::GameScore;
use pentanomial::{PairScore, Pentanomial};

use crate::game::Verdict;

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
        let half_points = 2 * self.wins.count() + self.draws.count();
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

    const WHITE_WIN: Verdict = Verdict::Win(Color::White);
    const BLACK_WIN: Verdict = Verdict::Win(Color::Black);
    const TALLIED: [(&[(Verdict, Verdict)], &str); 4] = [
        (
            &[(WHITE_WIN, BLACK_WIN); 4],
            "+8 =0 -0 8/8 Elo +1320 (+1320 to +1320) decisive",
        ),
        (
            &[(BLACK_WIN, WHITE_WIN); 4],
            "+0 =0 -8 0/8 Elo -1320 (-1320 to -1320) decisive",
        ),
        (
            &[
                (WHITE_WIN, BLACK_WIN),
                (BLACK_WIN, WHITE_WIN),
                (WHITE_WIN, WHITE_WIN),
                (BLACK_WIN, BLACK_WIN),
            ],
            "+4 =0 -4 4/8 Elo +0 (-297 to +297) inconclusive",
        ),
        (
            &[(WHITE_WIN, BLACK_WIN), (WHITE_WIN, Verdict::Draw)],
            "+3 =1 -0 3.5/4 Elo +338 (+149 to +1320) decisive",
        ),
    ];

    #[test]
    fn a_tally_counts_from_the_challenger_side_and_its_elo_bound_is_decisive_only_away_from_even() {
        for (pairs, written) in TALLIED {
            let tally = pairs
                .iter()
                .fold(Tally::default(), |tally, (as_white, as_black)| {
                    tally.recorded_pair(*as_white, *as_black)
                });
            assert_eq!(tally.to_string(), written);
        }
    }
}
