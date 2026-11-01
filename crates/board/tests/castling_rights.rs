use board::{Board, CastlingRights};
use fen::{DashOr, FenError};
use proptest::prelude::any;

const RIGHTS: [&str; 16] = [
    "-", "K", "Q", "KQ", "k", "Kk", "Qk", "KQk", "q", "Kq", "Qq", "KQq", "kq", "Kkq", "Qkq", "KQkq",
];
const REJECTED: [&str; 2] = ["KK", "x"];

struct Fixture;

impl Fixture {
    fn position(rights: &str) -> String {
        format!("r3k2r/8/8/8/8/8/8/R3K2R w {rights} - 0 1")
    }
}

#[test]
fn any_set_of_rights_prints_as_a_fen_field_that_parses_back() {
    for rights in RIGHTS {
        let text = Fixture::position(rights);
        assert_eq!(text.parse::<Board>().unwrap().to_string(), text);
    }
}

#[test]
fn repeated_or_unknown_letters_are_rejected() {
    for rights in REJECTED {
        assert_eq!(
            Fixture::position(rights).parse::<Board>(),
            Err(FenError::CastlingRights),
            "{rights}"
        );
    }
}

laws::roundtrip!(
    any_castling_rights_print_as_a_fen_field_and_parse_back_to_themselves,
    any::<CastlingRights>(),
    |rights: &CastlingRights| DashOr::from(*rights).to_string(),
    |text: &String| text
        .parse::<DashOr<CastlingRights>>()
        .map(CastlingRights::from),
    Ok
);
