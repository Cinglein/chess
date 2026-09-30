mod finished;
mod outcome;
mod record;
mod repetition_count;
mod streaks;
mod termination;

pub use finished::Finished;
pub use outcome::Verdict;

use std::iter;
use std::ops::ControlFlow;
use std::time::Instant;

use board::{Board, Color, HalfmoveClock, State};
use uci::{Clock, GoLimits, Position};

use crate::opponent::Opponent;
use crate::rules::Rules;
use outcome::Outcome;
use record::Record;
use repetition_count::RepetitionCount;
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
    const FIFTY_MOVES: HalfmoveClock = HalfmoveClock::new(100);
    const REPETITIONS: RepetitionCount = RepetitionCount::new(3);

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
        if let Err(error) = white.begin_game() {
            return Finished::new(
                self.record,
                Outcome::new(Color::White, Termination::Failure(error)),
            );
        }
        if let Err(error) = black.begin_game() {
            return Finished::new(
                self.record,
                Outcome::new(Color::Black, Termination::Failure(error)),
            );
        }
        match iter::repeat(()).try_fold(self, |game, ()| game.advance(white, black)) {
            ControlFlow::Break(finished) => finished,
            ControlFlow::Continue(game) => Finished::new(
                game.record,
                Outcome::new(game.board.side_to_move(), Termination::MoveLimit),
            ),
        }
    }

    fn advance(
        self,
        white: &mut dyn Opponent,
        black: &mut dyn Opponent,
    ) -> ControlFlow<Finished, Game> {
        if let Some(termination) = self.natural_end() {
            return self.ended(termination);
        }
        let side = self.board.side_to_move();
        let position = Position::played(*self.record.start(), self.record.moves());
        let started = Instant::now();
        let chosen = match side {
            Color::White => white.choose_move(position, GoLimits::Clock(self.clock)),
            Color::Black => black.choose_move(position, GoLimits::Clock(self.clock)),
        };
        let spent = started.elapsed();
        let chosen = match chosen {
            Ok(chosen) => chosen,
            Err(error) => return self.ended(Termination::Failure(error)),
        };
        let notation = chosen.notation();
        if spent > self.clock.remaining(side) {
            return self.ended(Termination::TimeForfeit);
        }
        let Some(board) = self
            .board
            .resolve_move(notation)
            .and_then(|chess_move| self.board.make_move(chess_move))
        else {
            return self.ended(Termination::IllegalMove);
        };
        ControlFlow::Continue(Game {
            board,
            record: self.record.extended(notation, board),
            clock: self.clock.minus_spent_plus_increment(side, spent),
            streaks: self.streaks.after(side, chosen.score(), &self.rules),
            ..self
        })
    }

    fn natural_end(&self) -> Option<Termination> {
        if self.board.fullmove_number() > self.rules.longest_game() {
            return Some(Termination::MoveLimit);
        }
        if self
            .rules
            .resign()
            .reached(self.streaks.losing(self.board.side_to_move()))
        {
            return Some(Termination::Resignation);
        }
        if self
            .rules
            .draw()
            .reached(self.streaks.level(), self.board.fullmove_number())
        {
            return Some(Termination::DrawAdjudicated);
        }
        if self.board.legal_moves().is_empty() {
            return Some(if self.board.in_check() {
                Termination::Checkmate
            } else {
                Termination::Stalemate
            });
        }
        if self.board.halfmove_clock() >= Self::FIFTY_MOVES {
            return Some(Termination::FiftyMoves);
        }
        if self.record.repetitions(self.board.hash()) >= Self::REPETITIONS {
            return Some(Termination::Repetition);
        }
        self.board
            .placement()
            .lacks_mating_material()
            .then_some(Termination::InsufficientMaterial)
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use board::Board;
    use uci::Clock;

    use super::Game;
    use crate::in_process_engine::InProcessEngine;
    use crate::rules::Rules;

    const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
    const BARE_KINGS: &str = "8/8/8/4k3/8/8/8/4K3 w - - 0 1";
    const QUICK: Duration = Duration::from_millis(64);
    const MATED: &str = "1-0 {checkmate}";
    const DEAD: &str = "1/2-1/2 {insufficient material}";

    impl Game {
        fn played_between_two_engines(fen: &str) -> String {
            let board: Board = fen.parse().unwrap();
            Game::from(board)
                .ruled_by(Rules::DEFAULT.timed(Clock::new(
                    QUICK,
                    QUICK,
                    Duration::ZERO,
                    Duration::ZERO,
                )))
                .play(
                    &mut InProcessEngine::default(),
                    &mut InProcessEngine::default(),
                )
                .outcome()
                .to_string()
        }
    }

    #[test]
    fn the_engine_delivers_a_mate_in_one_and_the_game_records_the_checkmate() {
        assert_eq!(Game::played_between_two_engines(MATE_IN_ONE), MATED);
    }

    #[test]
    fn bare_kings_end_the_game_before_anyone_moves() {
        assert_eq!(Game::played_between_two_engines(BARE_KINGS), DEAD);
    }
}
