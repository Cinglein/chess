use arena::{Openings, Settings};
use board::Board;
use settings::EmbeddedDefaults;
#[test]
fn every_opening_parses_and_offers_a_legal_move_and_a_round_repeats_them_all() {
    let boards: Vec<Board> = Openings::boards().collect();
    assert!(!boards.is_empty());
    assert!(boards.iter().all(|board| !board.legal_moves().is_empty()));
    let rounds = Settings::defaults().rounds();
    assert_eq!(
        Openings::repeated(rounds).len(),
        usize::from(rounds) * boards.len()
    );
}
