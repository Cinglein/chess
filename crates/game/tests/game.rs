use board::{Board, Color, FullmoveNumber, LongAlgebraic, NodeCount, PlyCount};
use eval::Score;
use game::{
    ArenaError, ChosenMove, DrawAdjudication, Finished, Game, Opponent, ResignAdjudication, Rules,
};
use opponent_ports::InProcessEngine;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::num::NonZeroU16;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use uci::{Clock, GoLimits, Increment, Position, Remaining};
const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
const QUICK: Rules = Rules::at_fixed_nodes(NodeCount::new(64));
const OPENING_MOVE: &str = "e2e4";
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
const A_MILLISECOND: Duration = Duration::from_millis(1);
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
const FIXTURES: [(&str, Rules, &str, usize); 8] = [
    (MATE_IN_ONE, QUICK, "1-0 {checkmate}", 1),
    (MATE_IN_ONE, QUICK.timed(NO_TIME), "1-0 {checkmate}", 1),
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

struct InProcess;

impl InProcess {
    fn played_between_two_engines(fen: &str, rules: Rules) -> Finished {
        let board: Board = fen.parse().unwrap();
        Game::from(board).ruled_by(rules).play(
            &mut InProcessEngine::default(),
            &mut InProcessEngine::default(),
        )
    }
}

struct Script {
    moves: RefCell<VecDeque<LongAlgebraic>>,
}

impl FromStr for Script {
    type Err = <LongAlgebraic as FromStr>::Err;

    fn from_str(text: &str) -> Result<Script, Self::Err> {
        text.split_whitespace()
            .map(str::parse)
            .collect::<Result<VecDeque<LongAlgebraic>, Self::Err>>()
            .map(|moves| Script {
                moves: RefCell::new(moves),
            })
    }
}

impl Opponent for &Script {
    fn begin_game(&mut self) -> Result<(), ArenaError> {
        Ok(())
    }

    fn choose_move(&mut self, _: Position<'_>, _: GoLimits) -> Result<ChosenMove, ArenaError> {
        self.moves
            .borrow_mut()
            .pop_front()
            .map(|notation| ChosenMove::new(notation, None))
            .ok_or(ArenaError::NoMove)
    }
}

struct SlowMover;

impl Opponent for SlowMover {
    fn begin_game(&mut self) -> Result<(), ArenaError> {
        Ok(())
    }

    fn choose_move(&mut self, _: Position<'_>, _: GoLimits) -> Result<ChosenMove, ArenaError> {
        thread::sleep(A_MILLISECOND);
        let notation = OPENING_MOVE.parse().map_err(|_| ArenaError::NoMove)?;
        Ok(ChosenMove::new(notation, None))
    }
}

#[derive(Default)]
struct ClockReader {
    seen: Option<Clock>,
}

impl Opponent for ClockReader {
    fn begin_game(&mut self) -> Result<(), ArenaError> {
        Ok(())
    }

    fn choose_move(&mut self, _: Position<'_>, limits: GoLimits) -> Result<ChosenMove, ArenaError> {
        self.seen = limits.clock();
        Err(ArenaError::NoMove)
    }
}

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
    let mut reader = ClockReader::default();
    let game = Game::from(Board::START).ruled_by(Rules::DEFAULT.timed(GENEROUS));
    let _finished = game.play(&mut SlowMover, &mut reader);
    let after = reader.seen.unwrap();
    let [charged, waiting] = [MOVER, WAITER].map(|side| after.time::<Remaining>(side));
    let before = GENEROUS.time::<Remaining>(MOVER);
    let increment = GENEROUS.time::<Increment>(MOVER);
    assert!(
        charged > before && charged < before + increment,
        "{charged:?}"
    );
    assert_eq!(waiting, GENEROUS.time::<Remaining>(WAITER));
}
