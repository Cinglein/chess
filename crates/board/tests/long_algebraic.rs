use board::{Board, LongAlgebraic};
use proptest::prelude::*;
const POSITIONS: [&str; 4] = [
    "r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1",
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
];

#[test]
fn any_notation_prints_and_parses_back_to_itself() {
    proptest!(|(notation: LongAlgebraic)| {
        prop_assert_eq!(notation.to_string().parse(), Ok(notation));
    });
}

#[test]
fn every_legal_move_roundtrips_through_its_text_and_resolves_to_itself() {
    for fen in POSITIONS {
        let board: Board = fen.parse().unwrap();
        for legal in board.legal_moves() {
            let notation = LongAlgebraic::from(legal);
            assert_eq!(legal.to_string(), notation.to_string());
            assert_eq!(board.resolve_move(notation), Some(legal), "{notation}");
        }
    }
}
