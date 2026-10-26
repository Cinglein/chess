mod finished;
mod label;
mod outcome;
mod record;
mod repetition_count;
#[cfg(test)]
mod script;
mod streaks;
mod termination;

pub use finished::Finished;
pub use label::Label;
pub use outcome::Verdict;

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

#[cfg(test)]
mod tests {
    use super::Game;
    use super::script::Script;
    use crate::fixtures::{InProcess, MATE_IN_ONE, QUICK};
    use crate::in_process_engine::InProcessEngine;
    use crate::rules::{DrawAdjudication, ResignAdjudication, Rules};
    use board::{Board, Color, FullmoveNumber, PlyCount};
    use eval::Score;
    use std::num::NonZeroU16;
    use std::time::Duration;
    use uci::{Clock, Increment, Remaining};
    const BARE_KINGS: &str = "8/8/8/4k3/8/8/8/4K3 w - - 0 1";
    const ROOK_EACH: &str = "4k3/r7/8/8/8/8/7R/4K3 w - - 0 1";
    const TWO_QUEENS: &str = "k7/8/8/8/8/8/8/4KQQ1 w - - 0 1";
    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const STALEMATE: &str = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
    const HUNDRED_HALFMOVES: &str = "4k3/8/8/8/8/8/8/4K2R w - - 100 60";
    const KNIGHTS_OUT_AND_BACK_TWICE: &str = "g1f3 g8f6 f3g1 f6g8 g1f3 g8f6 f3g1 f6g8";
    const THREEFOLD: &str = "position startpos moves g1f3 g8f6 f3g1 f6g8 g1f3 g8f6 f3g1 f6g8\n1/2-1/2 {threefold repetition}";
    const STALEMATED: &str = "position fen 7k/5Q2/6K1/8/8/8/8/8 b - - 0 1\n1/2-1/2 {stalemate}";
    const FIFTY_MOVES: &str =
        "position fen 4k3/8/8/8/8/8/8/4K2R w - - 100 60\n1/2-1/2 {fifty moves without progress}";
    const NO_MOVE: &str = "position startpos\n0-1 {engine failure: the engine had no move}";
    const A_NANOSECOND: Duration = Duration::from_nanos(1);
    const A_MINUTE: Duration = Duration::from_secs(60);
    const NO_TIME: Clock = Clock::new(A_NANOSECOND, A_NANOSECOND, Duration::ZERO, Duration::ZERO);
    const GENEROUS: Clock = Clock::new(A_MINUTE, A_MINUTE, Duration::from_secs(1), Duration::ZERO);
    const MOVER: Color = Color::White;
    const WAITER: Color = Color::Black;
    const ONE_MOVE: FullmoveNumber = FullmoveNumber::new(NonZeroU16::MIN);
    const TWO_MOVES: FullmoveNumber = FullmoveNumber::new(NonZeroU16::new(2).unwrap());
    const EIGHT_MOVES: FullmoveNumber = FullmoveNumber::new(NonZeroU16::new(8).unwrap());
    const LEVEL_FOR_FOUR_PLIES: DrawAdjudication =
        DrawAdjudication::new(ONE_MOVE, Score::INFINITY, PlyCount::new(4));
    const LOST_FOR_ONE_PLY: ResignAdjudication =
        ResignAdjudication::new(Score::new(64), PlyCount::new(1));
    const LEVEL_FOR_ONE_PLY: DrawAdjudication =
        DrawAdjudication::new(ONE_MOVE, Score::new(10), PlyCount::new(1));
    const DRAW_CLAIMABLE: Rules = QUICK.adjudicated_by(LEVEL_FOR_ONE_PLY, Rules::DEFAULT.resign());
    const DRAWN_WHEN_LEVEL: Rules = QUICK
        .lasting_at_most(EIGHT_MOVES)
        .adjudicated_by(LEVEL_FOR_FOUR_PLIES, Rules::DEFAULT.resign());
    const RESIGNABLE: Rules = QUICK.adjudicated_by(Rules::DEFAULT.draw(), LOST_FOR_ONE_PLY);
    const OUT_OF_TIME: Rules = Rules::DEFAULT.timed(NO_TIME).lasting_at_most(ONE_MOVE);
    const CUT_AT_TWO_MOVES: Rules = RESIGNABLE.lasting_at_most(TWO_MOVES);
    const FIXTURES: [(&str, Rules, &str, usize); 7] = [
        (MATE_IN_ONE, QUICK, "1-0 {checkmate}", 1),
        (MATE_IN_ONE, DRAW_CLAIMABLE, "1-0 {checkmate}", 1),
        (BARE_KINGS, QUICK, "1/2-1/2 {insufficient material}", 0),
        (ROOK_EACH, DRAWN_WHEN_LEVEL, "1/2-1/2 {draw adjudicated}", 4),
        (TWO_QUEENS, RESIGNABLE, "1-0 {resignation adjudicated}", 3),
        (START, OUT_OF_TIME, "0-1 {time forfeit}", 0),
        (START, CUT_AT_TWO_MOVES, "1/2-1/2 {move limit}", 4),
    ];
    const SCRIPTED: [(&str, &str, &str); 5] = [
        (START, KNIGHTS_OUT_AND_BACK_TWICE, THREEFOLD),
        (STALEMATE, "", STALEMATED),
        (HUNDRED_HALFMOVES, "", FIFTY_MOVES),
        (START, "e2e5", "position startpos\n0-1 {illegal move}"),
        (START, "", NO_MOVE),
    ];

    #[test]
    fn games_end_the_way_their_position_and_rules_dictate_and_every_label_carries_the_verdict() {
        for (fen, rules, outcome, plies) in FIXTURES {
            let finished = InProcess::played_between_two_engines(fen, rules);
            assert_eq!(
                (finished.outcome().to_string(), finished.labels().count()),
                (outcome.to_owned(), plies),
                "{fen}"
            );
            let verdict = finished.outcome().verdict();
            assert!(
                finished.labels().all(|label| label.verdict() == verdict
                    && verdict
                        .winner()
                        .is_none_or(|side| label.score_for(side) > label.score_for(!side))),
                "{fen}"
            );
        }
    }

    #[test]
    fn scripted_games_end_by_repetition_stalemate_fifty_moves_an_illegal_move_or_a_failure() {
        for (fen, moves, written) in SCRIPTED {
            let script: Script = moves.parse().unwrap();
            let board: Board = fen.parse().unwrap();
            let finished = Game::from(board).play(&mut &script, &mut &script);
            assert_eq!(finished.to_string(), written, "{fen}");
        }
    }

    #[test]
    fn a_played_move_charges_the_mover_for_its_thinking_time_and_adds_the_increment() {
        let engine = &mut InProcessEngine::default();
        let game = Game::from(Board::START).ruled_by(QUICK.timed(GENEROUS));
        let after = game
            .advance(engine, &mut InProcessEngine::default())
            .continue_value()
            .unwrap();
        let [charged, waiting] = [MOVER, WAITER].map(|side| after.clock.time::<Remaining>(side));
        let before = GENEROUS.time::<Remaining>(MOVER);
        let increment = GENEROUS.time::<Increment>(MOVER);
        assert!(
            charged > before && charged < before + increment,
            "{charged:?}"
        );
        assert_eq!(waiting, GENEROUS.time::<Remaining>(WAITER));
    }
}
