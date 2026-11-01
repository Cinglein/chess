mod white_result;

use std::io::Write;

use board::{Color, PieceKind};
use bulletformat::{BulletFormat, ChessBoard};
use game::Label;

use crate::datagen_error::DatagenError;
use crate::position_count::PositionCount;
use board::SetOperation;
use white_result::WhiteResult;

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
        let both = |kind| {
            placement.pieces(Color::White, kind).combined(
                placement.pieces(Color::Black, kind),
                SetOperation::DisjointUnion,
            )
        };
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
        ChessBoard::from_raw(
            bitboards,
            usize::from(label.board().side_to_move() == Color::Black),
            i16::try_from(label.score_for(Color::White).centipawns())?,
            WhiteResult::from(label.verdict()).fraction(),
        )
        .map(Example)
        .map_err(|_| DatagenError::Piece)
    }
}
