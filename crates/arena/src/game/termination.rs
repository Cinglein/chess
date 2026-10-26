use board::{Board, Color, HalfmoveClock};
use derive_more::Display;

use super::outcome::Verdict;
use super::record::Record;
use super::repetition_count::RepetitionCount;
use super::streaks::Streaks;
use crate::arena_error::ArenaError;
use crate::rules::Rules;

#[derive(Debug, Display)]
pub enum Termination {
    #[display("checkmate")]
    Checkmate,
    #[display("stalemate")]
    Stalemate,
    #[display("fifty moves without progress")]
    FiftyMoves,
    #[display("threefold repetition")]
    Repetition,
    #[display("insufficient material")]
    InsufficientMaterial,
    #[display("time forfeit")]
    TimeForfeit,
    #[display("illegal move")]
    IllegalMove,
    #[display("move limit")]
    MoveLimit,
    #[display("resignation adjudicated")]
    Resignation,
    #[display("draw adjudicated")]
    DrawAdjudicated,
    #[display("engine failure: {_0}")]
    Failure(ArenaError),
}

impl Termination {
    const FIFTY_MOVES: HalfmoveClock = HalfmoveClock::new(100);
    const REPETITIONS: RepetitionCount = RepetitionCount::new(3);

    pub(super) fn natural(
        board: &Board,
        rules: &Rules,
        streaks: Streaks,
        record: &Record,
    ) -> Option<Termination> {
        (board.fullmove_number() > rules.longest_game())
            .then_some(Termination::MoveLimit)
            .or_else(|| {
                rules
                    .resign()
                    .reached(streaks.losing(board.side_to_move()))
                    .then_some(Termination::Resignation)
            })
            .or_else(|| {
                rules
                    .draw()
                    .reached(streaks.level(), board.fullmove_number())
                    .then_some(Termination::DrawAdjudicated)
            })
            .or_else(|| {
                board
                    .legal_moves()
                    .is_empty()
                    .then_some(if board.in_check() {
                        Termination::Checkmate
                    } else {
                        Termination::Stalemate
                    })
            })
            .or_else(|| {
                (board.halfmove_clock() >= Self::FIFTY_MOVES).then_some(Termination::FiftyMoves)
            })
            .or_else(|| {
                (record.repetitions(board.hash()) >= Self::REPETITIONS)
                    .then_some(Termination::Repetition)
            })
            .or_else(|| {
                board
                    .placement()
                    .lacks_mating_material()
                    .then_some(Termination::InsufficientMaterial)
            })
    }

    #[must_use]
    pub fn verdict(&self, side_to_move: Color) -> Verdict {
        match self {
            Termination::Checkmate
            | Termination::Resignation
            | Termination::TimeForfeit
            | Termination::IllegalMove
            | Termination::Failure(_) => Verdict::Win(!side_to_move),
            Termination::Stalemate
            | Termination::FiftyMoves
            | Termination::Repetition
            | Termination::InsufficientMaterial
            | Termination::MoveLimit
            | Termination::DrawAdjudicated => Verdict::Draw,
        }
    }
}
