use board::Board;
use opponent_ports::Contestant;
use std::path::PathBuf;
use std::time::Duration;
use uci::{Elo, GoLimits, Position};

const PROGRAM: &str = env!("CARGO_BIN_EXE_chess");
const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
const MATING_MOVE: &str = "a1a8";
const THINK: GoLimits = GoLimits::MoveTime(Duration::from_millis(50));
const LIMIT: Elo = Elo::new(1500);

#[test]
fn the_chess_binary_completes_the_handshake_and_answers_a_timed_go_with_the_mate() {
    let mut engine = Contestant::Program(PathBuf::from(PROGRAM))
        .opponent(LIMIT)
        .unwrap();
    engine.begin_game().unwrap();
    let board: Board = MATE_IN_ONE.parse().unwrap();
    let chosen = engine
        .choose_move(Position::played(board, &[]), THINK)
        .unwrap();
    assert_eq!(chosen.notation().to_string(), MATING_MOVE);
}
