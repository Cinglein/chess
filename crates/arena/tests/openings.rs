use arena::{Openings, RoundCount};
use board::Board;
#[test]
fn every_opening_parses_and_offers_a_legal_move_and_a_round_repeats_them_all() {
    let boards: Vec<Board> = Openings::boards().collect();
    assert!(boards.iter().all(|board| !board.legal_moves().is_empty()));
    assert_eq!(
        Openings::repeated(RoundCount::from(2)).len(),
        2 * boards.len()
    );
}
