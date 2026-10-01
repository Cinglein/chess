mod finished;
mod label;
mod outcome;
mod record;
mod repetition_count;
mod streaks;
mod termination;

pub use finished::Finished;
pub use label::Label;
pub use outcome::Verdict;

use std::iter;
use std::ops::ControlFlow;
use std::time::Instant;

use board::{Board, Color, HalfmoveClock, State};
use uci::{Clock, Position};

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
        let limits = self.rules.thinking().limits(self.clock);
        let chosen = match side {
            Color::White => white.choose_move(position, limits),
            Color::Black => black.choose_move(position, limits),
        };
        let spent = started.elapsed();
        let chosen = match chosen {
            Ok(chosen) => chosen,
            Err(error) => return self.ended(Termination::Failure(error)),
        };
        let notation = chosen.notation();
        if self
            .rules
            .thinking()
            .forfeits(spent, self.clock.remaining(side))
        {
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
            record: self.record.extended(notation, board, chosen.score()),
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
    use std::num::NonZeroU16;
    use std::time::Duration;

    use board::{Board, Color, FullmoveNumber, NodeCount, PlyCount};
    use eval::Score;
    use uci::Clock;

    use super::{Finished, Game};
    use crate::in_process_engine::InProcessEngine;
    use crate::rules::{DrawAdjudication, ResignAdjudication, Rules, Thinking};

    const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
    const BARE_KINGS: &str = "8/8/8/4k3/8/8/8/4K3 w - - 0 1";
    const ROOK_EACH: &str = "4k3/r7/8/8/8/8/7R/4K3 w - - 0 1";
    const TWO_QUEENS_UP: &str = "k7/8/8/8/8/8/8/4KQQ1 w - - 0 1";
    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const QUICK: Rules = Rules::DEFAULT.thinking_by(Thinking::FixedNodes(NodeCount::new(64)));
    const NO_TIME: Clock = Clock::new(
        Duration::from_nanos(1),
        Duration::from_nanos(1),
        Duration::ZERO,
        Duration::ZERO,
    );
    const ONE_MOVE: FullmoveNumber = FullmoveNumber::new(NonZeroU16::MIN);
    const TWO_MOVES: FullmoveNumber = FullmoveNumber::new(NonZeroU16::new(2).unwrap());
    const EIGHT_MOVES: FullmoveNumber = FullmoveNumber::new(NonZeroU16::new(8).unwrap());
    const LEVEL_FOR_FOUR_PLIES: DrawAdjudication =
        DrawAdjudication::new(ONE_MOVE, Score::INFINITY, PlyCount::new(4));
    const LOST_FOR_ONE_PLY: ResignAdjudication =
        ResignAdjudication::new(Score::new(64), PlyCount::new(1));
    const FIXTURES: [(&str, Rules, &str, usize); 6] = [
        (MATE_IN_ONE, QUICK, "1-0 {checkmate}", 1),
        (BARE_KINGS, QUICK, "1/2-1/2 {insufficient material}", 0),
        (
            ROOK_EACH,
            QUICK
                .lasting_at_most(EIGHT_MOVES)
                .adjudicated_by(LEVEL_FOR_FOUR_PLIES, Rules::DEFAULT.resign()),
            "1/2-1/2 {draw adjudicated}",
            4,
        ),
        (
            TWO_QUEENS_UP,
            QUICK.adjudicated_by(Rules::DEFAULT.draw(), LOST_FOR_ONE_PLY),
            "1-0 {resignation adjudicated}",
            3,
        ),
        (
            START,
            Rules::DEFAULT.timed(NO_TIME).lasting_at_most(ONE_MOVE),
            "0-1 {time forfeit}",
            0,
        ),
        (
            START,
            QUICK.lasting_at_most(TWO_MOVES),
            "1/2-1/2 {move limit}",
            4,
        ),
    ];

    impl Game {
        fn played_between_two_engines(fen: &str, rules: Rules) -> Finished {
            let board: Board = fen.parse().unwrap();
            Game::from(board).ruled_by(rules).play(
                &mut InProcessEngine::default(),
                &mut InProcessEngine::default(),
            )
        }
    }

    #[test]
    fn games_end_the_way_their_position_and_rules_dictate() {
        for (fen, rules, outcome, plies) in FIXTURES {
            let finished = Game::played_between_two_engines(fen, rules);
            assert_eq!(
                (finished.outcome().to_string(), finished.labels().count()),
                (outcome.to_owned(), plies),
                "{fen}"
            );
        }
    }

    #[test]
    fn a_node_limited_game_labels_every_position_with_the_mover_score_and_the_verdict() {
        let finished = Game::played_between_two_engines(MATE_IN_ONE, QUICK);
        let label = finished.labels().next().unwrap();
        assert!(label.score_for(Color::White) > label.score_for(Color::Black));
        assert_eq!(label.verdict().winner(), Some(Color::White));
    }
}
