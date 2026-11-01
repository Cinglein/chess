use arena::{Label, Verdict};
use board::{Board, Color};
use bulletformat::{BulletFormat, ChessBoard};
use datagen::Example;
use eval::Score;
use proptest::prelude::*;
use std::str::FromStr;
struct Oracle;

impl Oracle {
    fn parsed(label: &Label) -> ChessBoard {
        let white_result = match label.verdict().winner() {
            Some(Color::White) => "1.0",
            Some(Color::Black) => "0.0",
            None => "0.5",
        };
        let line = format!(
            "{} | {} | {white_result}",
            label.board(),
            label.score_for(Color::White)
        );
        ChessBoard::from_str(&line).unwrap()
    }
}

#[test]
fn an_example_agrees_with_bullets_own_reading_of_the_position_as_text_and_writes_one_record() {
    proptest!(|(board: Board, score: Score, winner: Option<Color>)| {
        let label = Label::new(board, score, winner.map_or(Verdict::Draw, Verdict::Win));
        let mut written = Vec::new();
        let count = Example::write_labels([label].into_iter(), &mut written).unwrap();
        prop_assert_eq!(count.count(), 1);
        let parsed = [Oracle::parsed(&label)];
        prop_assert_eq!(written.as_slice(), ChessBoard::as_bytes_slice(&parsed));
    });
}
