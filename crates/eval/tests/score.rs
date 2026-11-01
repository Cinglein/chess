use core::num::{NonZeroI8, NonZeroI32};
use eval::Score;
use proptest::prelude::*;
const PLIES: u8 = 3;
const TWO_MOVES: NonZeroI32 = NonZeroI32::new(2).unwrap();

#[test]
fn a_mate_distance_in_moves_survives_the_trip_through_a_score_and_a_quicker_mate_scores_higher() {
    proptest!(|(moves: NonZeroI8)| {
        let moves = NonZeroI32::from(moves);
        prop_assert_eq!(
            (Score::mating_in_moves(moves).mate_in_moves(), (-Score::mating_in_moves(moves)).mate_in_moves()),
            (Some(moves.get()), Some(-moves.get()))
        );
        if let Some(later) = NonZeroI32::new(moves.get() + 1) {
            prop_assert!(Score::mating_in_moves(moves) > Score::mating_in_moves(later));
        }
    });
    assert_eq!(Score::mating_in_moves(TWO_MOVES), Score::mate_in(PLIES));
}

#[test]
fn centipawns_never_read_as_a_mate_and_a_table_scores_each_entry() {
    proptest!(|(score: Score, centipawns: [i32; 4])| {
        prop_assert_eq!(score.mate_in_moves().is_none(), score > -Score::LONGEST_MATE && score < Score::LONGEST_MATE);
        prop_assert_eq!(Score::new(score.centipawns()), score);
        prop_assert!(Score::table(centipawns).iter().zip(centipawns).all(|(scored, raw)| *scored == Score::new(raw)));
    });
}
