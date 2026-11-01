use board::{Board, LongAlgebraic};
use proptest::prelude::*;
use uci::Position;
#[test]
fn a_played_position_prints_as_a_line_that_parses_to_the_same_start_and_moves() {
    proptest!(|(board: Board, played in proptest::collection::vec(any::<LongAlgebraic>(), 0..8))| {
        let line = Position::played(board, &played).to_string();
        let parsed = Position::try_from(line.as_str()).unwrap();
        prop_assert_eq!(parsed.start(), board);
        prop_assert_eq!(parsed.moves().iter().collect::<Result<Vec<_>, _>>().unwrap(), played);
    });
}
