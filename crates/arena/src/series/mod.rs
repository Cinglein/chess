mod openings;
mod tally;

use board::{Board, Color};
use openings::Openings;
use tally::Tally;

use crate::game::{Finished, Game};
use crate::opponent::Opponent;
use crate::rules::Rules;

pub struct Series {
    challenger: Box<dyn Opponent>,
    reference: Box<dyn Opponent>,
    rules: Rules,
}

impl Series {
    #[must_use]
    pub fn new(
        challenger: Box<dyn Opponent>,
        reference: Box<dyn Opponent>,
        rules: Rules,
    ) -> Series {
        Series {
            challenger,
            reference,
            rules,
        }
    }

    pub fn play(mut self, rounds: usize, report: &mut dyn FnMut(&Finished)) -> Tally {
        let openings: Vec<Board> = (0..rounds).flat_map(|_| Openings::boards()).collect();
        openings
            .into_iter()
            .fold(Tally::default(), |tally, opening| {
                let as_white = Game::from(opening)
                    .ruled_by(self.rules)
                    .play(self.challenger.as_mut(), self.reference.as_mut());
                report(&as_white);
                let as_black = Game::from(opening)
                    .ruled_by(self.rules)
                    .play(self.reference.as_mut(), self.challenger.as_mut());
                report(&as_black);
                tally
                    .recorded(as_white.outcome().verdict(), Color::White)
                    .recorded(as_black.outcome().verdict(), Color::Black)
            })
    }
}
