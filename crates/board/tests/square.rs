use board::Square;

const UNPARSEABLE: [&str; 5] = ["e9", "i1", "e", "", "e44"];

#[test]
fn malformed_square_text_is_rejected() {
    for text in UNPARSEABLE {
        assert!(text.parse::<Square>().is_err(), "{text}");
    }
}
