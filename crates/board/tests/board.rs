use board::{Board, ChessMove, LongAlgebraic, MoveKind, Square};
use fen::FenError;
use proptest::prelude::*;
const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const ROUNDTRIPS: [&str; 2] = [
    "rnbqkbnr/pppp1ppp/8/4p3/4PP2/8/PPPP2PP/RNBQKBNR b KQkq f3 0 2",
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
];
const REJECTED: [(&str, &str, FenError); 12] = [
    ("/RNBQKBNR ", " ", FenError::RankCount),
    ("/pppppppp/", "/ppppppp/", FenError::RankWidth),
    ("/8/8/8/8/", "/9/8/8/8/", FenError::RankWidth),
    ("RNBQKBNR w", "RNBQKBNX w", FenError::Piece('X')),
    (" 0 1", " 0", FenError::FieldCount),
    (" 0 1", " 0 1 extra", FenError::FieldCount),
    (" w ", " x ", FenError::SideToMove),
    ("KQkq", "KQkx", FenError::CastlingRights),
    (" - ", " z9 ", FenError::EnPassant),
    (" - ", " e3 ", FenError::EnPassant),
    (" 0 1", " -1 1", FenError::HalfmoveClock),
    (" 0 1", " 0 0", FenError::FullmoveNumber),
];
const TRANSITIONS: [(&str, &str, &str, bool); 7] = [
    (
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "e2e4",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        false,
    ),
    (
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        "g8f6",
        "rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2",
        false,
    ),
    (
        "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
        "e1g1",
        "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
        false,
    ),
    (
        "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
        "e8c8",
        "2kr3r/8/8/8/8/8/8/R4RK1 w - - 2 2",
        false,
    ),
    (
        "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
        "a1a8",
        "R3k2r/8/8/8/8/8/8/4K2R b Kk - 0 1",
        true,
    ),
    (
        "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
        "e5d6",
        "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3",
        true,
    ),
    (
        "r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1",
        "b7a8q",
        "Q3k3/8/8/8/8/8/8/4K3 b - - 0 1",
        true,
    ),
];

struct Fixture;

impl Fixture {
    fn resolved(board: &Board, text: &str) -> ChessMove {
        let notation: LongAlgebraic = text.parse().unwrap();
        board.resolve_move(notation).unwrap()
    }
}

#[test]
fn positions_roundtrip_through_fen_and_malformed_fields_are_named() {
    assert_eq!(START.parse::<Board>(), Ok(Board::START));
    for text in [START].into_iter().chain(ROUNDTRIPS) {
        assert_eq!(text.parse::<Board>().unwrap().to_string(), text);
    }
    for (valid, broken, error) in REJECTED {
        let text = START.replace(valid, broken);
        assert_eq!(text.parse::<Board>(), Err(error), "{text}");
    }
}

#[test]
fn making_a_move_produces_the_position_fen_describes() {
    for (before, text, after, captures) in TRANSITIONS {
        let board: Board = before.parse().unwrap();
        let chess_move = Fixture::resolved(&board, text);
        let played = board.make_move(chess_move).unwrap();
        assert_eq!(played.to_string(), after, "{before} {chess_move}");
        assert_eq!(played.hash(), after.parse::<Board>().unwrap().hash());
        assert_eq!(chess_move.captures(board.placement()), captures);
    }
}

#[test]
fn any_reachable_position_roundtrips_through_fen_and_its_hash_matches_a_fresh_parse() {
    proptest!(|(board: Board)| {
        let reparsed: Board = board.to_string().parse().unwrap();
        prop_assert_eq!(reparsed, board);
        prop_assert_eq!(board.en_passant_file(), board.en_passant_square().map(Square::file));
        for chess_move in board.legal_moves() {
            let next = board.make_move(chess_move).unwrap();
            prop_assert_eq!(next.to_string().parse::<Board>().unwrap().hash(), next.hash());
        }
    });
}
