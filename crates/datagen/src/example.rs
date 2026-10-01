use std::io::Write;

use arena::Label;
use board::{Color, PieceKind};
use bulletformat::{BulletFormat, ChessBoard};

use crate::datagen_error::DatagenError;
use crate::position_count::PositionCount;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Example(ChessBoard);

impl Example {
    pub fn write_labels<W: Write>(
        labels: impl Iterator<Item = Label>,
        output: &mut W,
    ) -> Result<PositionCount, DatagenError> {
        let records = labels
            .map(|label| Example::try_from(label).map(|example| example.0))
            .collect::<Result<Vec<ChessBoard>, DatagenError>>()?;
        output.write_all(ChessBoard::as_bytes_slice(&records))?;
        Ok(PositionCount::new(records.len()))
    }
}

impl TryFrom<Label> for Example {
    type Error = DatagenError;

    fn try_from(label: Label) -> Result<Example, DatagenError> {
        let placement = label.board().placement();
        let both =
            |kind| placement.pieces(Color::White, kind) | placement.pieces(Color::Black, kind);
        let bitboards = [
            placement.occupied_by(Color::White).bits(),
            placement.occupied_by(Color::Black).bits(),
            both(PieceKind::Pawn).bits(),
            both(PieceKind::Knight).bits(),
            both(PieceKind::Bishop).bits(),
            both(PieceKind::Rook).bits(),
            both(PieceKind::Queen).bits(),
            both(PieceKind::King).bits(),
        ];
        let white_result = match label.verdict().winner() {
            Some(Color::White) => 1.0,
            Some(Color::Black) => 0.0,
            None => 0.5,
        };
        ChessBoard::from_raw(
            bitboards,
            usize::from(label.board().side_to_move() == Color::Black),
            i16::try_from(label.score_for(Color::White).centipawns())?,
            white_result,
        )
        .map(Example)
        .map_err(|_| DatagenError::Piece)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use arena::{Label, Verdict};
    use board::{Board, Color};
    use bulletformat::ChessBoard;
    use eval::Score;
    use proptest::prelude::*;

    use super::Example;

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
            prop_assert_eq!(Example::try_from(label).unwrap().0, Oracle::parsed(&label));
            let mut written = Vec::new();
            let count = Example::write_labels([label].into_iter(), &mut written).unwrap();
            prop_assert_eq!((count.count(), written.len()), (1, size_of::<ChessBoard>()));
        });
    }
}
