mod finished;
mod label;
mod outcome;
mod record;
mod repetition_count;
mod streaks;
mod termination;

pub use finished::Finished;
pub use label::Label;

use std::iter;
use std::ops::ControlFlow;
use std::time::{Duration, Instant};

use board::{Board, Color, State};
use uci::{Clock, Position, Remaining};

use crate::chosen_move::ChosenMove;
use crate::flag::Flag;
use crate::opponent::Opponent;
use crate::rules::Rules;
use outcome::Outcome;
use record::Record;
use streaks::Streaks;
use termination::Termination;

pub struct Game {
    board: Board,
    record: Record,
    clock: Clock,
    rules: Rules,
    streaks: Streaks,
}

impl State for Game {}

impl Game {
    #[must_use]
    pub fn ruled_by(self, rules: Rules) -> Game {
        Game {
            clock: rules.clock(),
            rules,
            ..self
        }
    }

    #[must_use]
    pub fn play(self, white: &mut dyn Opponent, black: &mut dyn Opponent) -> Finished {
        let opened = white
            .begin_game()
            .map_err(|error| Outcome::new(Color::White, Termination::Failure(error)))
            .and_then(|()| {
                black
                    .begin_game()
                    .map_err(|error| Outcome::new(Color::Black, Termination::Failure(error)))
            });
        match opened {
            Err(outcome) => Finished::new(self.record, outcome),
            Ok(()) => {
                match iter::repeat(()).try_fold(self, |game, ()| game.advance(white, black)) {
                    ControlFlow::Break(finished) => finished,
                    ControlFlow::Continue(game) => Finished::new(
                        game.record,
                        Outcome::new(game.board.side_to_move(), Termination::MoveLimit),
                    ),
                }
            }
        }
    }

    fn advance(
        self,
        white: &mut dyn Opponent,
        black: &mut dyn Opponent,
    ) -> ControlFlow<Finished, Game> {
        if let Some(termination) =
            Termination::natural(&self.board, &self.rules, self.streaks, &self.record)
        {
            self.ended(termination)
        } else {
            let position = Position::played(*self.record.start(), self.record.moves());
            let limits = self.rules.thinking().limits(self.clock);
            let started = Instant::now();
            let chosen = match self.board.side_to_move() {
                Color::White => white.choose_move(position, limits),
                Color::Black => black.choose_move(position, limits),
            };
            match chosen {
                Ok(chosen) => self.played(chosen, started.elapsed()),
                Err(error) => self.ended(Termination::Failure(error)),
            }
        }
    }

    fn played(self, chosen: ChosenMove, spent: Duration) -> ControlFlow<Finished, Game> {
        let side = self.board.side_to_move();
        let notation = chosen.notation();
        let timely = match self
            .rules
            .thinking()
            .flag(spent, self.clock.time::<Remaining>(side))
        {
            Flag::Standing => Ok(()),
            Flag::Fallen => Err(Termination::TimeForfeit),
        };
        let reached = self
            .board
            .resolve_move(notation)
            .and_then(|chess_move| self.board.make_move(chess_move))
            .ok_or(Termination::IllegalMove);
        match timely.and(reached) {
            Err(termination) => self.ended(termination),
            Ok(board) => ControlFlow::Continue(Game {
                board,
                record: self.record.extended(notation, board, chosen.score()),
                clock: self.clock.minus_spent_plus_increment(side, spent),
                streaks: self.streaks.after(side, chosen.score(), &self.rules),
                ..self
            }),
        }
    }

    fn ended(self, termination: Termination) -> ControlFlow<Finished, Game> {
        ControlFlow::Break(Finished::new(
            self.record,
            Outcome::new(self.board.side_to_move(), termination),
        ))
    }
}

impl From<Board> for Game {
    fn from(board: Board) -> Game {
        Game {
            board,
            record: Record::new(board),
            clock: Rules::DEFAULT.clock(),
            rules: Rules::DEFAULT,
            streaks: Streaks::default(),
        }
    }
}
