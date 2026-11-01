use board::Square;

const UNPARSEABLE: [&str; 5] = ["e9", "i1", "e", "", "e44"];
const MIRRORED: [(Square, Square); 3] = [
    (Square::A1, Square::A8),
    (Square::E2, Square::E7),
    (Square::H8, Square::H1),
];

#[test]
fn malformed_square_text_is_rejected() {
    for text in UNPARSEABLE {
        assert!(text.parse::<Square>().is_err(), "{text}");
    }
}

#[test]
fn a_mirrored_square_keeps_its_file_and_trades_its_rank() {
    for (square, mirrored) in MIRRORED {
        assert_eq!(square.mirrored(), mirrored);
    }
}
